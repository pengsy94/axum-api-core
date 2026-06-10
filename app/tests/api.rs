//! API 集成测试
//!
//! 直接用 axum Router 调用（不启动 HTTP 服务器）。

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use tower::ServiceExt;

use app::route;
use kernel::config::AppConfig;

fn setup() {
    unsafe {
        std::env::set_var("DEBUG", "false");
        std::env::set_var("SERVER_HOST", "127.0.0.1");
        std::env::set_var("SERVER_PORT", "0");
        std::env::set_var("LOG_LEVEL", "ERROR");
        std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        std::env::set_var("DATABASE_URL", "");
    }
    let _ = AppConfig::init();
}

fn build_app() -> axum::Router {
    setup();
    route::build_router()
}

#[tokio::test]
async fn test_health_returns_200() {
    let app = build_app();
    let res = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json: Value = serde_json::from_slice(
        &axum::body::to_bytes(res.into_body(), 1024).await.unwrap(),
    )
    .unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_root_returns_welcome() {
    let app = build_app();
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_404_returns_not_found() {
    let app = build_app();
    let res = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_login_with_valid_credentials() {
    let app = build_app();
    let body = serde_json::json!({
        "email": "admin@example.com",
        "password": "admin123",
    });
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json: Value = serde_json::from_slice(
        &axum::body::to_bytes(res.into_body(), 8192).await.unwrap(),
    )
    .unwrap();
    assert_eq!(json["code"], 200);
    assert!(json["data"]["token"].as_str().unwrap().len() > 20);
}

#[tokio::test]
async fn test_login_with_invalid_password() {
    let app = build_app();
    let body = serde_json::json!({
        "email": "admin@example.com",
        "password": "wrong-password",
    });
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json: Value = serde_json::from_slice(
        &axum::body::to_bytes(res.into_body(), 8192).await.unwrap(),
    )
    .unwrap();
    assert_eq!(json["code"], 401);
}

#[tokio::test]
async fn test_login_with_wrong_email() {
    let app = build_app();
    let body = serde_json::json!({
        "email": "hacker@example.com",
        "password": "admin123",
    });
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json: Value = serde_json::from_slice(
        &axum::body::to_bytes(res.into_body(), 8192).await.unwrap(),
    )
    .unwrap();
    assert_eq!(json["code"], 401);
}

#[tokio::test]
async fn test_user_routes_registered_in_test_build() {
    let app = build_app();
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/users?page=1&page_size=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_query_validation_returns_http_200_and_body_code_400() {
    let app = build_app();
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/users?page=0&page_size=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let json: Value = serde_json::from_slice(
        &axum::body::to_bytes(res.into_body(), 8192).await.unwrap(),
    )
    .unwrap();
    assert_eq!(json["code"], 400);
    assert_eq!(json["message"], "Query 参数校验失败");
}

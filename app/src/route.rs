use crate::api;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::{
    Router, middleware,
    routing::{get, post},
};
use kernel::config::server_config;
use ::middleware::request::{logging_middleware, rate_limiter, trace_middleware};

#[cfg(feature = "openapi")]
use utoipa::OpenApi;

pub fn build_router() -> Router {
    let config = server_config();

    let mut router = Router::new();

    // ws服务
    if config.ws_open {
        use crate::websocket::models::ConnectionManager;
        // 创建连接管理器
        let connection_manager = Arc::new(ConnectionManager::new());
        router = router.nest(
            &config.ws_path,
            crate::websocket::set_websocket_api(connection_manager),
        );
    }

    if config.debug {
        //  测试模块
        router = router.nest("/test", api::case::set_test_api());
    }

    // OpenAPI JSON 规范 + Swagger UI（编译时启用：cargo run --features openapi）
    #[cfg(feature = "openapi")]
    {
        use crate::docs::ApiDoc;

        // 原始 OpenAPI JSON
        router = router.route("/api/openapi.json", get(|| async {
            axum::Json(ApiDoc::openapi())
        }));

        // Swagger UI（通过 CDN 加载）
        router = router.route("/docs", get(swagger_ui));
    }

    // 健康检查
    router = router.route("/health", get(health_check));

    // 添加 API 路由
    router = add_api_routes(router);

    // 请求追踪（最外层中间件，确保 trace_id 覆盖所有请求）
    router = router.layer(middleware::from_fn(trace_middleware));

    if config.log_enable_oper_log {
        // 整体记录请求
        router = router.layer(middleware::from_fn(logging_middleware));
    }

    if config.rate_limit_enabled {
        router = router.layer(middleware::from_fn(rate_limiter));
    }

    router.fallback(handle_404)
}

fn add_api_routes(router: Router) -> Router {
    let mut router = router
        .route("/", get(index).post(index))
        .nest("/index", Router::new().route("/", get(index)))
        .nest(
            "/api",
            Router::new().route("/login", post(api::system::login)),
        );

    // 资源路由示例（debug 模式下可用）
    #[cfg(debug_assertions)]
    let router = resources!(router, "/api/users", api::user, [index, show, create, update, delete]);

    router
}

async fn index() -> &'static str {
    "Welcome to Axum Api Core!"
}

/// 存活检查（无状态，始终返回 ok）
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

/// Swagger UI 页面（通过 CDN 加载，无需额外依赖）
#[cfg(feature = "openapi")]
async fn swagger_ui() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../static/swagger.html"))
}

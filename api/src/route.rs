use crate::routes;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, post};
use common::config::{RESOURCE_DIR, WEB_STATIC_DIR, server_config};
use core::engine;
use database::repository::sys_user;
use tower_http::services::ServeDir;

pub fn build_router() -> Router {
    let mut router = Router::new();

    // 添加 Web 路由
    router = add_web_routes(router);

    // 静态资源
    let static_dir = format!("{}/{}", RESOURCE_DIR, WEB_STATIC_DIR);
    router.nest_service("/static", get_service(ServeDir::new(static_dir)))
}

fn add_web_routes(mut router: Router) -> Router {
    let config = server_config();

    if config.ws_open {
        router = router.nest(&config.ws_path, routes::websocket::set_websocket_api());
    }

    if config.debug {
        //  测试模块
        router = router.nest(
            "/test",
            Router::new()
                // websocket 页面
                .route("/ws", get(websocket))
                // 获取参数 /{id}
                .route("/{id}", get(routes::case::args::sys_path_test))
                .route("/{name}/{age}", get(routes::case::args::sys_path_2_test))
                .route("/query", get(routes::case::args::sys_query_test))
                // header获取
                .route("/header", get(routes::case::args::sys_header_test))
                // 返回json
                .route("/json", get(routes::case::args::sys_response_json))
                // post json提交参数
                .route("/post-json", post(routes::case::args::sys_query_json))
                // post form提交参数
                .route("/post-form", post(routes::case::args::sys_query_form)),
        );
    }

    router = router.route("/", get(index)).nest(
        "/api",
        Router::new().route("/", get(|| async { "Hello, Api!" })),
    );

    router
}

async fn index() -> impl IntoResponse {
    let sys_user_data = sys_user::get_by_id("2").await.unwrap();

    match engine::blade::render_with_struct("index", &sys_user_data) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn websocket() -> impl IntoResponse {
    let config = server_config();

    let ws_url = format!("ws://127.0.0.1:{}{}", config.port, config.ws_path);
    let data = std::collections::HashMap::from([("ws_url", ws_url.as_str())]);

    match engine::blade::render_map("websocket/index", data) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

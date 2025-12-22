use crate::routes;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use common::config::{RESOURCE_DIR, WEB_STATIC_DIR, server_config};
use core::engine;
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
        // router = router.nest("/test", test_api());
    }

    router = router
        .route("/", get(index))
        .route("/websocket", get(websocket))
        .nest(
            "/api",
            Router::new().route("/", get(|| async { "Hello, Api!" })),
        );

    router
}

async fn index() -> impl IntoResponse {
    use std::collections::HashMap;
    let data = HashMap::from([
        ("title", "首页 - Axum - Blade 模板示例"),
        ("welcome_message", "欢迎来到 Axum - Blade 模板世界"),
    ]);

    match engine::blade::render_map("index", data) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn websocket() -> impl IntoResponse {
    match engine::blade::render_view("websocket/index") {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

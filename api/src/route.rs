use crate::routes;

use axum::Router;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use common::config::{RESOURCE_DIR, WEB_STATIC_DIR, server_config};
use core::engine;
use orm::repository::sys_user;
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
        router = router.nest("/test", Router::new().route("/ws", get(websocket)));
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

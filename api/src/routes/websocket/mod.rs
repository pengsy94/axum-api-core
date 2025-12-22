use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::routes::websocket::models::ConnectionManager;

pub mod models;
pub mod ws;

// websocket api 路由
pub fn set_websocket_api() -> Router {
    // 创建连接管理器
    let connection_manager = Arc::new(ConnectionManager::new());

    Router::new()
        .route("/", get(ws::websocket_handler))
        .with_state(connection_manager)
}

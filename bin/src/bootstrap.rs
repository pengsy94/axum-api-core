use api::route;
use axum::{
    Router,
    http::{Method, StatusCode},
};
use common::config::{AppConfig, server_config};
use orm::DatabaseManager;
use std::net::SocketAddr;
use tower_http::{
    compression::{CompressionLayer, DefaultPredicate, Predicate, predicate::NotForContentType},
    cors::{Any, CorsLayer},
};

pub async fn make() -> anyhow::Result<(Router, SocketAddr)> {
    // 初始化配置（只调用一次）
    AppConfig::init()?;
    // 构建应用
    let (app, listener) = build_router().await?;
    // 初始化数据库信息
    DatabaseManager::init().await?;

    // 打印系统信息
    core::system::show();

    Ok((app, listener))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn build_router() -> anyhow::Result<(Router, SocketAddr)> {
    let config = server_config();

    let app = route::build_router().fallback(handle_404);
    let app = match &config.content_gzip {
        true => {
            //  开启压缩后 SSE 数据无法返回  text/event-stream 单独处理不压缩
            let predicate =
                DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
            app.layer(CompressionLayer::new().compress_when(predicate))
        }
        false => app,
    };
    // 添加cors跨越
    let app = app.layer(setup_cors());

    let listener: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    Ok((app, listener))
}

fn setup_cors() -> CorsLayer {
    let methods = vec![Method::GET, Method::HEAD, Method::OPTIONS];

    CorsLayer::new()
        .allow_methods(methods)
        .allow_origin(Any)
        .allow_headers(Any)
}

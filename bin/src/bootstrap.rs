use axum::Router;
use axum::http::Method;
use axum::http::StatusCode;
use kernel::config::AppConfig;
use kernel::config::server_config;
use kernel::tasks::manager::SchedulerManager;
use database::DatabaseManager;
use http::route;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::compression::DefaultPredicate;
use tower_http::compression::Predicate;
use tower_http::compression::predicate::NotForContentType;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub async fn make() -> anyhow::Result<(Router, TcpListener, SchedulerManager)> {
    // 初始化配置（只调用一次）
    AppConfig::init()?;
    // 构建应用
    let (app, listener) = build_application().await?;
    // 初始化数据库信息
    DatabaseManager::init().await?;
    // 打印系统信息
    kernel::system::show();
    // 创建调度器管理器
    let scheduler_manager = SchedulerManager::new();
    // 启动定时任务
    scheduler_manager.start().await.unwrap();

    Ok((app, listener, scheduler_manager))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn build_application() -> anyhow::Result<(Router, TcpListener)> {
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

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(addr).await?;
    Ok((app, listener))
}

fn setup_cors() -> CorsLayer {
    let methods = vec![Method::GET, Method::POST, Method::HEAD, Method::OPTIONS];

    CorsLayer::new()
        .allow_methods(methods)
        .allow_origin(Any)
        .allow_headers(Any)
}

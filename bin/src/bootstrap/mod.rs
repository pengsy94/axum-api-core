use app::route;
use axum::Router;
use axum::http::{HeaderValue, Method, StatusCode};
use std::sync::OnceLock;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

static PROMETHEUS_HANDLE: OnceLock<metrics_exporter_prometheus::PrometheusHandle> = OnceLock::new();
use axum::response::Json;
use axum::routing::get;
use database::DatabaseManager;
use serde_json::json;
use kernel::config::AppConfig;
use kernel::config::server_config;
use kernel::tasks::manager::SchedulerManager;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::compression::DefaultPredicate;
use tower_http::compression::Predicate;
use tower_http::compression::predicate::NotForContentType;
use tower_http::cors::AllowOrigin;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub mod logger;

pub async fn make() -> anyhow::Result<(Router, TcpListener, SchedulerManager)> {
    // 初始化配置（只调用一次）
    AppConfig::init()?;
    // 构建应用
    let (make_service, listener) = build_application().await?;
    // 初始化数据库信息
    if let Err(e) = DatabaseManager::init().await {
        tracing::warn!("数据库初始化失败（服务仍可运行）: {}", e);
    }
    // 初始化 Prometheus 指标收集
    let handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .expect("Prometheus 初始化失败");
    PROMETHEUS_HANDLE.set(handle).ok();

    // 打印系统信息
    kernel::system::show();
    // 创建调度器管理器
    let scheduler_manager = SchedulerManager::new();
    // 启动定时任务
    scheduler_manager.start().await.unwrap();

    Ok((make_service, listener, scheduler_manager))
}

async fn build_application() -> anyhow::Result<(Router, TcpListener)> {
    let config = server_config();

    let app = route::build_router();
    let app = match &config.content_gzip {
        true => {
            //  开启压缩后 SSE 数据无法返回  text/event-stream 单独处理不压缩
            let predicate =
                DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
            app.layer(CompressionLayer::new().compress_when(predicate))
        }
        false => app,
    };

    // 请求超时中间件
    let app = match config.request_timeout_seconds {
        0 => app,
        secs => app.layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(secs))),
    };

    // 添加 /metrics 端点
    let app = app.route("/metrics", get(metrics_handler));

    // 添加cors跨越
    let make_service = app.layer(setup_cors());

    // 就绪检查（依赖数据库连接）
    let make_service = make_service
        .route("/ready", get(ready_check));

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(addr).await?;
    Ok((make_service, listener))
}

/// Prometheus 指标导出端点
async fn metrics_handler() -> String {
    PROMETHEUS_HANDLE
        .get()
        .map(|h| h.render())
        .unwrap_or_else(|| "# metrics not initialized".to_string())
}

/// 就绪检查：验证服务是否就绪
async fn ready_check() -> (StatusCode, Json<serde_json::Value>) {
    if kernel::config::database_config().enabled {
        // 启用了数据库：检查连接池
        if database::get_db().is_some() {
            (StatusCode::OK, Json(json!({ "status": "ready", "database": "connected" })))
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "status": "not_ready", "database": "disconnected" })))
        }
    } else {
        // 未启用数据库：仅检查服务存活
        (StatusCode::OK, Json(json!({ "status": "ready", "database": "disabled" })))
    }
}

fn setup_cors() -> CorsLayer {
    let config = server_config();
    let methods = vec![Method::GET, Method::POST, Method::HEAD, Method::OPTIONS];

    if config.cors_allowed_origins == "*" {
        CorsLayer::new()
            .allow_methods(methods)
            .allow_origin(Any)
            .allow_headers(Any)
    } else {
        let origins = config
            .cors_allowed_origins
            .split(',')
            .map(|s| s.trim().parse::<HeaderValue>().expect("无效的 CORS_ALLOWED_ORIGINS 值"))
            .collect::<Vec<_>>();
        CorsLayer::new()
            .allow_methods(methods)
            .allow_origin(AllowOrigin::list(origins))
            .allow_headers(Any)
    }
}

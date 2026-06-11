use app::route;
use axum::Router;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::Json;
use axum::routing::get;
use database::{DatabaseManager, RedisManager};
use kernel::config::AppConfig;
use kernel::config::server_config;
use kernel::tasks::manager::SchedulerManager;
use serde_json::json;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::compression::DefaultPredicate;
use tower_http::compression::Predicate;
use tower_http::compression::predicate::NotForContentType;
use tower_http::cors::AllowOrigin;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

pub mod logger;

static PROMETHEUS_HANDLE: OnceLock<metrics_exporter_prometheus::PrometheusHandle> = OnceLock::new();

pub async fn make() -> anyhow::Result<(Router, TcpListener, SchedulerManager)> {
    // 初始化配置（只调用一次）
    if AppConfig::try_global().is_none() {
        AppConfig::init()?;
    }

    // 打印系统信息
    kernel::system::show();

    // 构建应用
    let (make_service, listener) = build_application().await?;
    // 初始化数据库信息（带 10 秒超时）
    match tokio::time::timeout(Duration::from_secs(10), DatabaseManager::init()).await {
        Ok(Ok(())) => {}
        Ok(Err(e)) => println!("⚠️ 数据库初始化失败（服务仍可运行）: {}", e),
        Err(_) => println!("⚠️ 数据库连接超时（10s），跳过初始化（服务仍可运行）"),
    }
    // 初始化 Redis（带 10 秒超时）
    match tokio::time::timeout(Duration::from_secs(10), RedisManager::init()).await {
        Ok(Ok(())) => {}
        Ok(Err(e)) => println!("⚠️ Redis 初始化失败（服务仍可运行）: {}", e),
        Err(_) => println!("⚠️ Redis 连接超时（10s），跳过初始化（服务仍可运行）"),
    }
    // 初始化 Prometheus 指标收集
    let handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .expect("Prometheus 初始化失败");
    PROMETHEUS_HANDLE.set(handle).ok();

    // 创建调度器管理器
    let scheduler_manager = SchedulerManager::new();
    // 启动定时任务
    if let Err(e) = scheduler_manager.start().await {
        tracing::warn!(error = %e, "cron 定时任务启动失败，服务继续运行");
    }

    println!();
    println!("{:>2} Axum service has started successfully!!!", "🎉🎉🎉");
    println!();

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
        secs => app.layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(secs),
        )),
    };

    // 添加 /metrics 端点
    let app = app.route("/metrics", get(metrics_handler));

    // 添加cors跨越
    let make_service = app.layer(setup_cors());

    // 就绪检查（依赖数据库连接）
    let make_service = make_service.route("/ready", get(ready_check));

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
    let db_status = if kernel::config::database_config().enabled {
        if DatabaseManager::ping().await {
            "connected"
        } else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "status": "not_ready", "database": "disconnected" })),
            );
        }
    } else {
        "disabled"
    };

    let redis_status = if kernel::config::redis_config().enabled {
        if RedisManager::ping().await {
            "connected"
        } else {
            "disconnected"
        }
    } else {
        "disabled"
    };

    (
        StatusCode::OK,
        Json(json!({ "status": "ready", "database": db_status, "redis": redis_status })),
    )
}

fn setup_cors() -> CorsLayer {
    let config = server_config();
    let methods = vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
    ];

    if config.cors_allowed_origins == "*" {
        CorsLayer::new()
            .allow_methods(methods)
            .allow_origin(Any)
            .allow_headers(Any)
    } else {
        let origins = config
            .cors_allowed_origins
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<HeaderValue>()
                    .expect("无效的 CORS_ALLOWED_ORIGINS 值")
            })
            .collect::<Vec<_>>();
        CorsLayer::new()
            .allow_methods(methods)
            .allow_origin(AllowOrigin::list(origins))
            .allow_headers(Any)
    }
}

use axum::{
    extract::{OriginalUri, Request},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;
use tracing::{info, info_span, Instrument};
use uuid::Uuid;



pub async fn logging_middleware(
    OriginalUri(original_uri): OriginalUri, // 原始地址
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let headers = request.headers().clone();
    // 记录请求开始时间
    let start = Instant::now();
    // 打印请求信息
    info!(
        "[Request] {} {} - Headers: {:?}",
        method, original_uri, headers
    );

    // 处理请求
    let response = next.run(request).await;
    // 记录响应信息
    let duration = start.elapsed();

    info!(
        "[Response] {} {} - Status: {} - Duration: {:?}",
        method,
        original_uri,
        response.status(),
        duration
    );

    response
}

/// 限流，每秒超过100个就延迟，每秒自动重置计数器
pub async fn rate_limiter(request: Request, next: Next) -> Result<Response, StatusCode> {
    // 简单的计数器限流
    static REQUEST_COUNT: AtomicU32 = AtomicU32::new(0);
    const MAX_REQUESTS: u32 = 100;

    // 一次性启动后台任务，每秒重置计数器
    static RESET_INIT: OnceLock<()> = OnceLock::new();
    RESET_INIT.get_or_init(|| {
        tokio::spawn(async {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                REQUEST_COUNT.store(0, Ordering::Relaxed);
            }
        });
    });

    let current = REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    if current >= MAX_REQUESTS {
        // 超过阈值时延迟 100ms 以缓解压力
        sleep(Duration::from_millis(100)).await;
    }

    Ok(next.run(request).await)
}

/// 请求追踪 ID（可通过 `Extension` 在 handler 中取出）
#[derive(Debug, Clone)]
pub struct TraceId(pub String);

/// 请求追踪中间件：为每个请求注入 TraceId，日志 span 和响应头
pub async fn trace_middleware(mut request: Request, next: Next) -> Response {
    let trace_id = Uuid::new_v4().to_string();

    // 存入 request extensions，handler 可取出
    request.extensions_mut().insert(TraceId(trace_id.clone()));

    // 创建 tracing span
    let span = info_span!("request", trace_id = %trace_id);
    let mut response = async { next.run(request).await }.instrument(span).await;

    // 在响应头中返回 trace_id
    response.headers_mut().insert(
        http::header::HeaderName::from_static("x-trace-id"),
        HeaderValue::from_str(&trace_id).unwrap(),
    );

    response
}

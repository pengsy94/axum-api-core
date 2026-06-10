use axum::{
    extract::{OriginalUri, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use common::utils::response::ResJsonString;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;
use tracing::info;
use uuid::Uuid;

/// 响应体日志最大字节数（超出截断）
const MAX_BODY_LOG_BYTES: usize = 4096;

/// 请求日志展示的 header
const REQ_HDRS: &[&str] = &["content-type", "authorization", "user-agent"];

/// 敏感头
const SENSITIVE: &[&str] = &["authorization", "cookie", "set-cookie"];

/// 格式化核心 headers 为紧凑字符串（name: value, ...）
fn fmt_hdrs(hdrs: &HeaderMap, keys: &[&str]) -> String {
    let parts: Vec<String> = hdrs
        .iter()
        .filter(|(name, _)| keys.contains(&name.as_str().to_lowercase().as_str()))
        .map(|(name, value)| {
            let n = name.as_str().to_lowercase();
            let v = value.to_str().unwrap_or("-");
            let v = if SENSITIVE.contains(&n.as_str()) {
                if let Some(prefix) = v.split_whitespace().next() {
                    format!("{} ***", prefix)
                } else {
                    "***".to_string()
                }
            } else {
                v.to_string()
            };
            format!("{}: {}", n, v)
        })
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("  |  {}", parts.join("  |  "))
    }
}

fn fmt_dur(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", d.as_secs_f64())
    }
}

pub async fn logging_middleware(
    OriginalUri(original_uri): OriginalUri,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let headers = request.headers().clone();
    let start = Instant::now();
    let trace_id = request
        .extensions()
        .get::<TraceId>()
        .map(|t| t.0.clone())
        .unwrap_or_else(|| "-".to_string());

    info!(
        "\n┌─ HTTP REQUEST ─────────────────────────────────────────\n\
         │ trace_id: {}\n\
         │ method  : {}\n\
         │ uri     : {}\n\
         │ headers : {}\n\
         └────────────────────────────────────────────────────────",
        trace_id,
        method,
        original_uri,
        fmt_hdrs(&headers, REQ_HDRS)
    );

    let response = next.run(request).await;
    let duration = start.elapsed();
    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("");

    let log_body = response
        .extensions()
        .get::<ResJsonString>()
        .map(|body| truncate_log_body(&body.0))
        .unwrap_or_else(|| "<streaming-or-non-json-body>".to_string());

    info!(
        "\n┌─ HTTP RESPONSE ────────────────────────────────────────\n\
         │ trace_id: {}\n\
         │ status  : {} {}\n\
         │ cost    : {}\n\
         │ body    : {}\n\
         └────────────────────────────────────────────────────────",
        trace_id,
        status.as_u16(),
        status_text,
        fmt_dur(duration),
        log_body
    );

    response
}

/// 限流中间件
pub async fn rate_limiter(request: Request, next: Next) -> Result<Response, StatusCode> {
    static REQUEST_COUNT: AtomicU32 = AtomicU32::new(0);
    const MAX_REQUESTS: u32 = 100;

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
        sleep(Duration::from_millis(100)).await;
    }
    Ok(next.run(request).await)
}

/// Prometheus 指标中间件
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().to_string();
    let uri = request.uri().path().to_string();

    let response = next.run(request).await;

    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    metrics::counter!("http_requests_total", "method" => method.clone(), "path" => uri.clone(), "status" => status.clone()).increment(1);
    metrics::histogram!("http_request_duration_seconds", "method" => method, "path" => uri).record(duration);

    response
}

/// 请求追踪 ID
#[derive(Debug, Clone)]
pub struct TraceId(pub String);

/// 请求追踪中间件
pub async fn trace_middleware(mut request: Request, next: Next) -> Response {
    let trace_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(TraceId(trace_id.clone()));

    let mut response = next.run(request).await;

    response.headers_mut().insert(
        http::HeaderName::from_static("x-trace-id"),
        http::HeaderValue::from_str(&trace_id).unwrap(),
    );

    response
}

fn truncate_log_body(body: &str) -> String {
    if body.len() > MAX_BODY_LOG_BYTES {
        let truncated: String = body.chars().take(MAX_BODY_LOG_BYTES).collect();
        format!("{}... ({} bytes, truncated)", truncated, body.len())
    } else {
        body.to_string()
    }
}

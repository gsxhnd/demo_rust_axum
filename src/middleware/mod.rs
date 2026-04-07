//! HTTP request tracing middleware
//!
//! This module provides middleware for tracing HTTP requests with OpenTelemetry.

use axum::{
    extract::Request,
    http::{header::HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

use crate::tracing::otel::{format_trace_context, get_current_span_id, get_current_trace_id};

/// 请求追踪中间件
///
/// 为每个 HTTP 请求创建追踪信息，并在响应头中添加 trace_id 和 span_id。
pub async fn trace_request(mut request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();

    // 获取或生成 trace context
    let trace_id = get_current_trace_id();
    let span_id = get_current_span_id();

    // 记录请求开始
    tracing::info!(
        "HTTP request started: {} {} {}",
        method,
        path,
        format_trace_context()
    );

    // 将 trace context 注入到请求扩展中
    request.extensions_mut().insert(RequestTraceContext {
        trace_id: trace_id.clone(),
        span_id: span_id.clone(),
    });

    // 处理请求
    let response = next.run(request).await;

    // 计算请求处理时间
    let duration = start.elapsed();

    // 获取响应状态码
    let status = response.status();

    // 记录请求完成
    let trace_context = format_trace_context();
    if status.is_server_error() {
        tracing::error!(
            "HTTP request completed: {} {} {} - {} in {:?}",
            method,
            path,
            status.as_u16(),
            trace_context,
            duration
        );
    } else if status.is_client_error() {
        tracing::warn!(
            "HTTP request completed: {} {} {} - {} in {:?}",
            method,
            path,
            status.as_u16(),
            trace_context,
            duration
        );
    } else {
        tracing::info!(
            "HTTP request completed: {} {} {} - {} in {:?}",
            method,
            path,
            status.as_u16(),
            trace_context,
            duration
        );
    }

    // 将 trace context 添加到响应头
    let mut response = response;
    if let Some(ref tid) = trace_id
        && let Ok(header_value) = HeaderValue::from_str(tid)
    {
        response.headers_mut().insert(TRACE_ID_HEADER.clone(), header_value);
    }
    if let Some(ref sid) = span_id
        && let Ok(header_value) = HeaderValue::from_str(sid)
    {
        response.headers_mut().insert(SPAN_ID_HEADER.clone(), header_value);
    }

    response
}

/// 请求追踪上下文
#[derive(Clone, Debug)]
pub struct RequestTraceContext {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

/// Trace ID 响应头名称
pub static TRACE_ID_HEADER: HeaderName = HeaderName::from_static("x-trace-id");

/// Span ID 响应头名称
pub static SPAN_ID_HEADER: HeaderName = HeaderName::from_static("x-span-id");

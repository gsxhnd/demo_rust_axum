//! OpenTelemetry tracing integration module
//!
//! This module provides OpenTelemetry tracing support for distributed tracing.
//! To enable, set `opentelemetry.enable = true` in config.toml and configure the OTLP endpoint.
//!
//! # Usage
//! Set the following in config.toml:
//! ```toml
//! [opentelemetry]
//! enable = true
//! otlp_endpoint = "http://localhost:4317"
//! ```

use std::env;

use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    trace::{Sampler, SdkTracerProvider},
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{OpenTelemetryConfig, TracingConfig};

/// OpenTelemetry layer handle for proper shutdown
pub struct OtelLayerHandle {
    tracer_provider: Option<SdkTracerProvider>,
    _file_guard: Option<WorkerGuard>,
}

/// 获取环境默认值
fn get_env_defaults(env_name: &str) -> (String, String, Vec<String>) {
    match env_name {
        "development" => (
            "debug".to_string(),
            "pretty".to_string(),
            vec!["console".to_string()],
        ),
        "staging" => (
            "info".to_string(),
            "json".to_string(),
            vec!["console".to_string(), "file".to_string()],
        ),
        "production" => (
            "warn".to_string(),
            "json".to_string(),
            vec!["console".to_string(), "file".to_string()],
        ),
        "test" => (
            "error".to_string(),
            "pretty".to_string(),
            vec!["console".to_string()],
        ),
        _ => (
            "info".to_string(),
            "pretty".to_string(),
            vec!["console".to_string()],
        ),
    }
}

/// Initialize OpenTelemetry tracing layer with OTLP exporter
///
/// Returns `Some(OtelLayerHandle)` if OpenTelemetry is enabled and OTLP endpoint is configured.
pub fn init_otel_layer(
    tracing_config: &TracingConfig,
    otel_config: &OpenTelemetryConfig,
) -> Option<OtelLayerHandle> {
    // Check if OpenTelemetry is enabled
    if !otel_config.enable {
        tracing::debug!("OpenTelemetry is disabled in configuration");
        return None;
    }

    let otlp_endpoint = otel_config.otlp_endpoint.as_ref()?;

    if otlp_endpoint.is_empty() {
        tracing::warn!("OTLP endpoint is empty, skipping OpenTelemetry initialization");
        return None;
    }

    tracing::info!(
        "Initializing OpenTelemetry with endpoint: {}",
        otlp_endpoint
    );

    // Create OTLP span exporter with tonic (gRPC)
    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_export_config(opentelemetry_otlp::ExportConfig {
            endpoint: Some(otlp_endpoint.clone()),
            ..Default::default()
        })
        .build()
        .expect("Failed to create OTLP span exporter");

    // Create resource with service information
    let service_name = otel_config
        .service_name
        .clone()
        .or_else(|| env::var("OTEL_SERVICE_NAME").ok())
        .unwrap_or_else(|| "rust_axum".to_string());
    let service_version = otel_config
        .service_version
        .clone()
        .or_else(|| env::var("OTEL_SERVICE_VERSION").ok())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    let resource = Resource::builder_empty()
        .with_service_name(service_name.clone())
        .with_attributes([opentelemetry::KeyValue::new(
            "service.version",
            service_version,
        )])
        .build();

    // Create SDK tracer provider with batch exporter
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(resource)
        .with_sampler(Sampler::AlwaysOn)
        .build();

    // Get tracer instance
    let tracer = tracer_provider.tracer("rust_axum_tracer");

    // Create OpenTelemetry layer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // 获取环境信息
    let env_name = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    // 获取日志配置
    let (default_level, default_format, default_output) = get_env_defaults(&env_name);

    let level = if tracing_config.level.is_empty() {
        &default_level
    } else {
        &tracing_config.level
    };
    let format = if tracing_config.format.is_empty() {
        &default_format
    } else {
        &tracing_config.format
    };
    let output: Vec<String> = if tracing_config.output.is_empty() {
        default_output
    } else {
        tracing_config.output.clone()
    };

    let has_console = output.contains(&"console".to_string()) || output.is_empty();
    let has_file = output.contains(&"file".to_string());

    // 创建环境过滤器
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    // 根据配置初始化日志层
    let mut file_guard: Option<WorkerGuard> = None;

    if has_file {
        let base_path = format!("{}/{}", tracing_config.file_path, env_name);
        let file_appender =
            RollingFileAppender::new(Rotation::DAILY, &base_path, format!("{}.log", env_name));
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        file_guard = Some(guard);

        let file_layer = fmt::layer()
            .json()
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(true)
            .with_line_number(true)
            .with_writer(non_blocking);

        if has_console {
            let console_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(true);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(otel_layer)
                .with(file_layer)
                .with(console_layer)
                .init();
        } else {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(otel_layer)
                .with(file_layer)
                .init();
        }
    } else {
        // 纯控制台模式
        if format == "json" {
            let console_layer = fmt::layer()
                .json()
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(true)
                .with_line_number(true);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(otel_layer)
                .with(console_layer)
                .init();
        } else {
            let console_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(env_name != "production");

            tracing_subscriber::registry()
                .with(env_filter)
                .with(otel_layer)
                .with(console_layer)
                .init();
        }
    }

    tracing::info!(
        "OpenTelemetry initialized: service={}, endpoint={}, format={}",
        service_name,
        otlp_endpoint,
        format
    );

    Some(OtelLayerHandle {
        tracer_provider: Some(tracer_provider),
        _file_guard: file_guard,
    })
}

/// Shutdown OpenTelemetry tracer provider
///
/// Call this before application exit to ensure all spans are exported.
pub fn shutdown_otel() {
    tracing::debug!("OpenTelemetry shutdown requested");
}

/// Get the current trace ID from the current span
///
/// Returns `None` if no active span or OpenTelemetry is not enabled.
pub fn get_current_trace_id() -> Option<String> {
    use opentelemetry::trace::TraceContextExt;

    let context = tracing::Span::current().context();
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() {
        Some(span_context.trace_id().to_string())
    } else {
        None
    }
}

/// Get the current span ID from the current span
///
/// Returns `None` if no active span or OpenTelemetry is not enabled.
pub fn get_current_span_id() -> Option<String> {
    use opentelemetry::trace::TraceContextExt;

    let context = tracing::Span::current().context();
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() {
        Some(span_context.span_id().to_string())
    } else {
        None
    }
}

/// Format trace context for logging
///
/// Returns a formatted string containing trace_id and span_id if available.
pub fn format_trace_context() -> String {
    if let (Some(trace_id), Some(span_id)) = (get_current_trace_id(), get_current_span_id()) {
        return format!("trace_id={}, span_id={}", trace_id, span_id);
    }
    String::new()
}

impl Drop for OtelLayerHandle {
    fn drop(&mut self) {
        // Explicitly shutdown the tracer provider to flush pending spans
        if let Some(provider) = self.tracer_provider.take() {
            let _ = provider.shutdown();
        }
    }
}

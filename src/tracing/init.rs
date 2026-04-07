use std::env;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::TracingConfig;

/// Tracing guards holder
#[derive(Default)]
pub struct TracingGuards {
    _file_guard: Option<WorkerGuard>,
}

/// 根据环境获取默认配置
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

/// Initialize tracing log system
///
/// This function initializes the basic logging layer. If OpenTelemetry is needed,
/// call `init_tracing` first and then `init_otel_layer` from the otel module.
pub fn init_tracing(config: &TracingConfig) -> Option<TracingGuards> {
    // Check if tracing is enabled
    if !config.enable {
        tracing::debug!("Tracing is disabled in configuration");
        return None;
    }

    let env_name = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    // 获取环境默认值
    let (default_level, default_format, default_output) = get_env_defaults(&env_name);

    // 使用配置值或默认值
    let level = if config.level.is_empty() {
        &default_level
    } else {
        &config.level
    };
    let format = if config.format.is_empty() {
        &default_format
    } else {
        &config.format
    };
    let output = if config.output.is_empty() {
        &default_output
    } else {
        &config.output
    };

    let has_console = output.contains(&"console".to_string()) || output.is_empty();
    let has_file = output.contains(&"file".to_string());

    // 创建环境过滤器
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    // 初始化日志
    if has_file {
        let base_path = format!("{}/{}", config.file_path, env_name);

        let file_appender =
            RollingFileAppender::new(Rotation::DAILY, &base_path, format!("{}.log", env_name));

        let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);

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
                .with(file_layer)
                .with(console_layer)
                .init();
        } else {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(file_layer)
                .init();
        }

        tracing::info!(
            "Tracing initialized: level={}, format=json, env={}, output=file",
            level,
            env_name
        );

        return Some(TracingGuards {
            _file_guard: Some(file_guard),
        });
    }

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
            .with(console_layer)
            .init();
    }

    tracing::info!(
        "Tracing initialized: level={}, format={}, env={}",
        level,
        format,
        env_name
    );

    None
}

use rust_axum::{AppState, Config, create_router, init_otel_layer, init_tracing, shutdown_otel};
use sea_orm::Database;
use tracing::info;

#[tokio::main]
async fn main() {
    // 加载配置
    let config = Config::load();
    let tracing_config = config.tracing_config();
    let otel_config = config.opentelemetry_config();

    // 初始化 tracing（日志系统）
    let _guards = init_tracing(&tracing_config);

    // 初始化 OpenTelemetry（分布式追踪）
    let _otel_layer = init_otel_layer(&tracing_config, &otel_config);

    info!("Starting Rust Axum server...");
    info!("Server configuration: {}", config.address());

    let db = match config.database_url() {
        Some(url) => {
            info!("Connecting to database...");
            Database::connect(&url)
                .await
                .expect("Failed to connect to database")
        }
        None => {
            panic!("Database URL not configured");
        }
    };

    if config.redis_url().is_some() {
        info!("Redis configured");
    }

    let app_state = AppState::new(db);
    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind(&config.address())
        .await
        .unwrap_or_else(|_| panic!("Failed to bind port {}", config.port));

    info!("🚀 Server running on http://{}", config.address());

    // 启动服务器
    let result = axum::serve(listener, app).await;

    // 关闭时清理 OpenTelemetry 资源
    shutdown_otel();

    result.expect("Server error");
}

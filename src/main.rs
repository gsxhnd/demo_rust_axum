use rust_axum::{AppState, Config, create_router};
use sea_orm::Database;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Rust Axum server...");

    let config = Config::load();
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
        .expect(&format!("Failed to bind port {}", config.port));

    info!("🚀 Server running on http://{}", config.address());

    axum::serve(listener, app).await.expect("Server error");
}

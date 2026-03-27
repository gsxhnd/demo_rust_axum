use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: Option<DatabaseConfig>,
    pub redis: Option<RedisConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            database: None,
            redis: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| format!("config/{}.toml", env));

        Self::load_from_path(&config_path)
    }

    pub fn load_from_path(path: &str) -> Self {
        let mut config = Config::default();

        if let Ok(config_str) = std::fs::read_to_string(path) {
            match toml::from_str::<Config>(&config_str) {
                Ok(parsed) => {
                    info!("Loaded configuration from {}", path);
                    config = parsed;
                }
                Err(e) => {
                    tracing::error!("Failed to parse config file: {}", e);
                }
            }
        } else {
            tracing::warn!("Config file not found: {}, using defaults", path);
        }

        config.host = std::env::var("HOST").unwrap_or(config.host);
        config.port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(config.port);

        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.database = Some(DatabaseConfig { url });
        }

        if let Ok(url) = std::env::var("REDIS_URL") {
            config.redis = Some(RedisConfig { url });
        }

        config
    }

    pub fn database_url(&self) -> Option<String> {
        self.database.as_ref().map(|db| db.url.clone())
    }

    pub fn redis_url(&self) -> Option<String> {
        self.redis.as_ref().map(|r| r.url.clone())
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

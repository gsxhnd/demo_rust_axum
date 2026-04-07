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
pub struct TracingConfig {
    pub enable: bool,
    pub level: String,
    pub format: String,
    pub output: Vec<String>,
    pub file_path: String,
    pub rotation_days: u32,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enable: true,
            level: "info".to_string(),
            format: "pretty".to_string(),
            output: vec!["console".to_string()],
            file_path: "logs".to_string(),
            rotation_days: 7,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenTelemetryConfig {
    pub enable: bool,
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub otlp_endpoint: Option<String>,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        Self {
            enable: false,
            service_name: None,
            service_version: None,
            otlp_endpoint: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: Option<DatabaseConfig>,
    pub redis: Option<RedisConfig>,
    pub tracing: Option<TracingConfig>,
    pub opentelemetry: Option<OpenTelemetryConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            database: None,
            redis: None,
            tracing: None,
            opentelemetry: None,
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

        // 环境变量覆盖 tracing 配置
        if config.tracing.is_none() {
            config.tracing = Some(TracingConfig::default());
        }
        if let Some(ref mut tc) = config.tracing {
            if let Ok(enable) = std::env::var("TRACING_ENABLE") {
                tc.enable = enable.parse().unwrap_or(true);
            }
            if let Ok(level) = std::env::var("TRACING_LEVEL") {
                tc.level = level;
            }
            if let Ok(format) = std::env::var("TRACING_FORMAT") {
                tc.format = format;
            }
            if let Ok(output) = std::env::var("TRACING_OUTPUT") {
                tc.output = output.split(',').map(|s| s.trim().to_string()).collect();
            }
        }

        // 环境变量覆盖 opentelemetry 配置
        if config.opentelemetry.is_none() {
            config.opentelemetry = Some(OpenTelemetryConfig::default());
        }
        if let Some(ref mut oc) = config.opentelemetry {
            if let Ok(enable) = std::env::var("OTEL_ENABLE") {
                oc.enable = enable.parse().unwrap_or(false);
            }
            if let Ok(service_name) = std::env::var("OTEL_SERVICE_NAME") {
                oc.service_name = Some(service_name);
            }
            if let Ok(service_version) = std::env::var("OTEL_SERVICE_VERSION") {
                oc.service_version = Some(service_version);
            }
            if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
                oc.otlp_endpoint = Some(endpoint);
            }
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

    pub fn tracing_config(&self) -> TracingConfig {
        self.tracing.clone().unwrap_or_default()
    }

    pub fn opentelemetry_config(&self) -> OpenTelemetryConfig {
        self.opentelemetry.clone().unwrap_or_default()
    }
}

# 配置说明

## 配置文件

配置文件位于 `config/` 目录：

| 文件 | 环境 |
|------|------|
| `config/dev.toml` | 开发环境 |
| `config/staging.toml` | 预发布环境 |
| `config/prod.toml` | 生产环境 |

通过 `APP_ENV` 环境变量选择配置文件，默认 `development`。也可通过 `CONFIG_PATH` 直接指定路径。

## 完整配置项

```toml
host = "127.0.0.1"
port = 3000

[database]
url = "postgresql://user:password@localhost:5432/dev_db"

[redis]
url = "redis://localhost:6379/0"

[tracing]
enable = true
level = "debug"          # trace / debug / info / warn / error
format = "pretty"        # pretty（彩色控制台）/ json（结构化）
output = ["console"]     # console / file（可多选）
file_path = "logs"       # 日志文件目录
rotation_days = 7        # 日志轮转天数

[opentelemetry]
enable = false
service_name = "rust_axum"
service_version = "0.1.0"
otlp_endpoint = "http://localhost:4317"
```

## 环境变量覆盖

环境变量优先级高于配置文件：

| 环境变量 | 覆盖项 |
|----------|--------|
| `APP_ENV` | 选择配置文件（默认 `development`） |
| `CONFIG_PATH` | 直接指定配置文件路径 |
| `HOST` | 监听地址 |
| `PORT` | 监听端口 |
| `DATABASE_URL` | 数据库连接字符串 |
| `REDIS_URL` | Redis 连接字符串 |
| `TRACING_ENABLE` | 是否启用日志 |
| `TRACING_LEVEL` | 日志级别 |
| `TRACING_FORMAT` | 日志格式 |
| `TRACING_OUTPUT` | 输出目标（逗号分隔） |
| `OTEL_ENABLE` | 是否启用 OpenTelemetry |
| `OTEL_SERVICE_NAME` | 服务名称 |
| `OTEL_SERVICE_VERSION` | 服务版本 |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OTLP 导出端点 |

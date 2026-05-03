# config 模块

**文件**：`src/config.rs`

## 职责

加载应用配置，支持 TOML 文件 + 环境变量覆盖。

## 核心类型

### `Config`

应用主配置结构体，字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| host | String | 监听地址，默认 `127.0.0.1` |
| port | u16 | 监听端口，默认 `3000` |
| database | Option\<DatabaseConfig\> | 数据库配置 |
| redis | Option\<RedisConfig\> | Redis 配置 |
| tracing | Option\<TracingConfig\> | 日志配置 |
| opentelemetry | Option\<OpenTelemetryConfig\> | OTel 配置 |

### 加载逻辑 — `Config::load()`

1. 读取 `APP_ENV` 环境变量（默认 `"development"`）
2. 拼接路径 `config/{APP_ENV}.toml`（`APP_ENV=development` → `config/development.toml`）
3. 也可通过 `CONFIG_PATH` 环境变量直接指定配置文件路径
4. 解析 TOML 文件，失败则使用默认值
5. 环境变量逐项覆盖：`HOST`, `PORT`, `DATABASE_URL`, `REDIS_URL`, `TRACING_*`, `OTEL_*`

### 注意

- `APP_ENV=development` 对应的文件是 `config/dev.toml`（文件名为 `dev` 而非 `development`）—— 实际上 `Config::load()` 会拼接 `config/development.toml`，但项目中只有 `dev.toml`。如果使用默认 `APP_ENV`，需要确保文件名匹配，或设置 `CONFIG_PATH=config/dev.toml`。
- 根目录的 `config.toml` 不会被自动加载。

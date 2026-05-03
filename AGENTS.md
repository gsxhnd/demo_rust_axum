# AI 编程助手指南

Rust + Axum 学习/实践项目。Rust 2024 Edition。

## 快速命令

```bash
cargo check                      # 快速编译检查
cargo fmt && cargo clippy -- -D warnings  # 提交前必须通过
cargo test                       # 运行所有测试（目前无测试）
APP_ENV=development cargo run    # 启动服务（需要 PostgreSQL）
```

## 运行前提

- **PostgreSQL 必须可用**：`main.rs` 中 `Database::connect` 失败会 panic
- 配置加载顺序：`config/{APP_ENV}.toml` → 环境变量覆盖（`HOST`, `PORT`, `DATABASE_URL`, `REDIS_URL` 等）
- 默认 `APP_ENV=development`，对应 `config/dev.toml`
- 根目录 `config.toml` 不会被自动加载，仅作参考；实际读取 `config/` 目录下的文件

## 项目结构要点

```
src/
├── main.rs          # 入口：加载配置 → 初始化 tracing → 连接 DB → 启动 server
├── lib.rs           # 公共导出（Config, AppState, AppError, create_router, tracing 工具）
├── config.rs        # TOML 配置 + 环境变量覆盖逻辑
├── state.rs         # AppState：持有 DatabaseConnection + Arc<RwLock<Vec<User>>>
├── routes.rs        # 路由定义，所有路由挂载 trace_request 中间件
├── error.rs         # AppError 枚举，实现 IntoResponse（返回 JSON {"error": "..."} ）
├── handlers/        # health.rs, user.rs — handler 直接用 SeaORM 操作数据库
├── models/user.rs   # SeaORM 手写 Entity（非 sea-orm-cli 生成），表名 "users"
├── middleware/       # trace_request：注入 x-trace-id / x-span-id 响应头
└── tracing/         # init.rs（tracing-subscriber 初始化）、otel.rs（OpenTelemetry 集成）
```

## 关键约定

- **SeaORM Entity 手写**：`models/user.rs` 手动实现 `Entity`, `Column`, `PrimaryKey`, `ColumnTrait` 等，不使用 `sea-orm-cli` 代码生成。新增模型时遵循同样模式。
- **SeaORM 2.0 RC**：依赖 `sea-orm = "2.0.0-rc.37"`，API 可能与 1.x 文档不同。
- **Axum 0.8 路由语法**：路径参数用 `{id}` 而非 `:id`（如 `/users/{id}`）。
- **AppState 通过 `.with_state()` 传递**：handler 用 `State(state): State<AppState>` 提取。
- **错误处理**：handler 返回 `impl IntoResponse`，内部用 `match` 处理 `Result`；`AppError` 可用但当前 handler 未统一使用。
- **导入分组**：标准库 → 外部 crate → `crate::` 内部模块，每组之间空一行。
- **异步锁**：使用 `tokio::sync::RwLock`，不要用 `std::sync::RwLock`。
- **日志**：使用 `tracing` crate（`info!`, `warn!`, `error!`），不要用 `println!` 或 `log` crate。

## 注意事项

- 项目目前无测试。添加测试时使用 `#[cfg(test)]` 模块。
- `middleware/mod.rs` 使用了 `let ... && let ...` 链式 let（需要 nightly 或 edition 2024 的 `let_chains` 特性）。
- OpenTelemetry 默认关闭（`opentelemetry.enable = false`），启用需配置 OTLP endpoint。
- `tracing/` 模块名与 `tracing` crate 同名，内部引用 crate 时需注意路径歧义。

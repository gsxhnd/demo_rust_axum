# 技术栈

## 核心依赖

| 依赖 | 版本 | 用途 | 备注 |
|------|------|------|------|
| Rust | 2024 Edition | 语言 | 支持 `let_chains` 等新特性 |
| axum | 0.8 | Web 框架 | 路径参数语法为 `{id}`（非 `:id`） |
| tokio | 1 (full) | 异步运行时 | |
| sea-orm | 2.0.0-rc.37 | ORM | RC 版本，API 与 1.x 不同 |
| serde / serde_json | 1 | 序列化 | |
| toml | 0.8 | 配置解析 | |
| tracing | 0.1 | 结构化日志 | 配合 tracing-subscriber 0.3 |
| tracing-opentelemetry | 0.32 | OTel 集成 | |
| opentelemetry | 0.31 | 分布式追踪 | OTLP gRPC 导出 |
| chrono | 0.4 | 时间处理 | 启用 serde feature |
| redis | 1 | Redis 客户端 | 当前仅配置，未深度使用 |

## 关键版本注意

- **SeaORM 2.0 RC**：处于 Release Candidate 阶段，API 可能与官方 1.x 文档不一致。遇到编译错误时优先查看 [SeaORM 2.0 迁移指南](https://www.sea-ql.org/SeaORM/)。
- **Axum 0.8**：路由路径参数从 `:id` 改为 `{id}`，这是与 0.7 的主要差异。
- **Rust 2024 Edition**：`middleware/mod.rs` 使用了 `let ... && let ...` 链式 let 语法。

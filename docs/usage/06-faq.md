# FAQ

## Q: 项目需要什么版本的 Rust？

需要支持 Rust 2024 Edition 的 toolchain。代码中使用了 `let_chains` 等 2024 Edition 特性。

## Q: 必须安装 Redis 吗？

不是必须的。Redis 配置为可选项（`Option<RedisConfig>`），未配置时服务仍可正常启动。当前代码中 Redis 未被实际使用。

## Q: 为什么没有数据库迁移？

项目处于学习阶段，暂未引入迁移工具。需要手动创建 `users` 表，参见 [快速开始](./01-getting-started.md)。

## Q: 如何添加新的 API 端点？

1. 在 `src/handlers/` 中编写 handler 函数
2. 在 `src/handlers/mod.rs` 中导出
3. 在 `src/routes.rs` 中注册路由

## Q: 如何添加新的数据模型？

参见 [领域模型文档](../dev/03-domain-model.md#新增模型指南)。本项目使用手写 SeaORM Entity，不使用代码生成工具。

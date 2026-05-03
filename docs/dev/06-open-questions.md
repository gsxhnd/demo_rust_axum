# 开放问题

## 待决策

### 1. 数据库迁移策略

当前无迁移工具。需要决定：
- 使用 SeaORM 内置的 migration 模块？
- 使用独立 SQL 脚本？
- `users` 表的 `created_at` 默认值由数据库还是应用层设置？

### 2. 错误处理统一

`AppError` 已定义但 handler 中未统一使用，仍以 `match` + 手动构造响应为主。是否迁移到 `Result<T, AppError>` 模式？

### 3. 内存用户列表的定位

`AppState.users: Arc<RwLock<Vec<User>>>` 与数据库中的 `users` 表并存，用途不明确。是否移除内存列表，完全依赖数据库？

### 4. Redis 使用场景

Redis 已配置但未实际使用。计划用于：
- 会话管理？
- 查询缓存？
- 速率限制？

### 5. `tracing` 模块命名冲突

`src/tracing/` 与 `tracing` crate 同名，内部引用时需要注意路径歧义。是否重命名为 `observability/` 或 `telemetry/`？

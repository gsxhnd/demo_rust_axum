# state 模块

**文件**：`src/state.rs`

## 职责

定义应用共享状态 `AppState`，通过 Axum 的 `.with_state()` 注入路由。

## 核心类型

### `AppState`

```rust
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,  // 内存用户列表（学习用途）
    pub db: DatabaseConnection,          // SeaORM 数据库连接
}
```

- `Clone` 派生是 Axum state 的要求
- `users` 使用 `tokio::sync::RwLock`（非 `std::sync::RwLock`），支持异步读写
- `db` 是 SeaORM 的 `DatabaseConnection`，内部已实现连接池

## 设计说明

`users` 字段目前未被 handler 使用（handler 直接操作数据库），保留作为内存状态管理的学习示例。

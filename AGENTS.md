# AI 编程助手指南

本文档为 AI 编程助手（Cursor、Copilot、Cline 等）提供项目开发指导。

## 项目概述

这是一个使用 **Rust + Axum** 构建的 Web API 服务。

**技术栈**：

- Rust 2024 Edition
- Axum 0.8（Web 框架）
- Tokio（异步运行时）
- SeaORM 2.0（ORM 框架）
- PostgreSQL（数据库）
- Redis（缓存）
- Serde（序列化）
- Tracing（日志）

## 项目结构

```
src/
├── main.rs           # 应用入口
├── lib.rs            # 公共模块导出
├── config.rs         # 配置管理
├── error.rs          # 错误类型定义
├── state.rs          # 应用状态
├── routes.rs         # 路由定义
├── handlers/         # 请求处理器
│   ├── mod.rs
│   ├── health.rs     # 健康检查
│   └── user.rs       # 用户相关处理
└── models/           # 数据模型
    ├── mod.rs
    └── user.rs       # 用户模型
```

## 构建、Lint 和测试命令

**开发：**
```bash
cargo run            # 运行应用
cargo check          # 快速检查编译错误
```

**构建：**
```bash
cargo build          # 开发构建
cargo build --release  # 生产构建
```

**格式化：**
```bash
cargo fmt            # 格式化代码
cargo fmt -- --check # 检查格式（不修改）
```

**Lint：**
```bash
cargo clippy         # 运行 Clippy 检查
cargo clippy -- -D warnings  # 将警告视为错误
```

**测试：**
```bash
cargo test           # 运行所有测试
cargo test -- --nocapture  # 显示测试输出
cargo test test_name # 运行单个测试（按名称过滤）
cargo test --lib     # 仅运行库测试
```

## 代码风格指南

### 导入规范

```rust
// 1. 标准库导入
use std::fmt;
use std::sync::Arc;

// 2. 外部 crate 导入
use axum::{Router, routing::get, Json};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn};

// 3. 内部模块导入（使用 crate::）
use crate::models::User;
use crate::state::AppState;
use crate::error::AppError;
```

### 格式化

- 使用 `cargo fmt` 默认格式（4 空格缩进）
- 最大行宽：100 字符
- 导入分组：标准库 → 外部 crate → 内部模块
- 每组导入之间空一行

### 命名约定

- **文件名**：snake_case（`user.rs`, `health_check.rs`）
- **结构体/枚举**：PascalCase（`User`, `AppError`, `CreateUserRequest`）
- **函数/方法**：snake_case（`get_users`, `create_user`）
- **变量/字段**：snake_case（`user_id`, `created_at`）
- **常量**：UPPER_SNAKE_CASE
- **类型参数**：PascalCase（`T`, `U`）

### 错误处理

使用自定义错误类型 `AppError`，实现 `IntoResponse`：

```rust
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

// Handler 返回 Result<T, AppError> 或直接 impl IntoResponse
pub async fn handler() -> Result<Json<User>, AppError> {
    // ...
}
```

**最佳实践**：
- 使用 `Result<T, AppError>` 作为 handler 返回类型
- 错误信息应具体且有描述性
- 使用 `tracing` 记录错误日志

### 日志

```rust
use tracing::{info, warn, error, debug};

// 在 handler 中记录关键操作
info!("Creating user: {} ({})", payload.name, payload.email);
warn!("User not found: {}", id);
error!("Database connection failed: {}", err);
```

### 处理器模式

```rust
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

// Handler 签名模式
pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.read().await;
    (StatusCode::OK, Json(users.to_vec()))
}

pub async fn get_user(Path(id): Path<u32>, State(state): State<AppState>) -> impl IntoResponse {
    // 使用模式匹配处理 Option/Result
    match find_user(id).await {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}
```

### 路由定义

```rust
use axum::{Router, routing::get};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).delete(delete_user))
        .with_state(state)
}
```

### 模型定义

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize)]  // 仅反序列化（请求体）
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
```

### 应用状态

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
}
```

**注意**：使用 `Arc<RwLock<T>>` 保证线程安全和异步读写。

## 常见模式

### 配置管理

```rust
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        Self { host, port }
    }
}
```

### 环境变量

- `HOST` - 服务器主机（默认：127.0.0.1）
- `PORT` - 服务器端口（默认：3000）
- `DATABASE_URL` - PostgreSQL 连接字符串
- `REDIS_URL` - Redis 连接字符串

## 注意事项

- 项目使用 Rust 2024 Edition
- 所有异步函数使用 `async/await` 模式
- 使用 `tokio::sync::RwLock` 而非 `std::sync::RwLock`
- 日志使用 `tracing` crate，初始化在 `main.rs`
- 无预配置测试，添加测试时使用 `#[cfg(test)]` 模块

# error 模块

**文件**：`src/error.rs`

## 职责

定义统一错误类型 `AppError`，实现 `IntoResponse` 以便直接作为 Axum 响应返回。

## 核心类型

### `AppError`

```rust
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}
```

### 响应格式

所有错误返回 JSON：

```json
{"error": "错误信息"}
```

HTTP 状态码映射：
- `NotFound` → 404
- `BadRequest` → 400
- `InternalServerError` → 500

## 当前使用状况

`AppError` 已定义并实现 `IntoResponse` + `Display`，但 handler 中尚未统一使用。当前 handler 通过 `match` 手动构造 `(StatusCode, &str).into_response()` 返回错误。

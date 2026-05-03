# handlers 模块

**目录**：`src/handlers/`

## 文件结构

- `mod.rs` — 导出所有 handler 函数
- `health.rs` — 健康检查端点
- `user.rs` — 用户 CRUD 操作

## Handler 签名模式

所有 handler 遵循统一模式：

```rust
pub async fn handler_name(
    State(state): State<AppState>,  // 提取共享状态
    // 可选：Path(id): Path<i32>, Json(payload): Json<T>
) -> impl IntoResponse {
    match some_operation(&state.db).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (StatusCode::XXX, "error message").into_response(),
    }
}
```

## user.rs 详情

| 函数 | 操作 | SeaORM 调用 |
|------|------|-------------|
| `get_users` | 查询全部 | `User::find().all(&db)` |
| `create_user` | 插入 | `ActiveModel { ... }.insert(&db)` |
| `get_user` | 按 ID 查询 | `User::find_by_id(id).one(&db)` |
| `delete_user` | 按 ID 删除 | `User::delete_by_id(id).exec(&db)` |

## 注意

- handler 直接操作 `state.db`，无 service 层抽象
- 错误处理使用 `match` + 手动构造响应，未使用 `AppError`
- `delete_user` 通过 `rows_affected > 0` 判断是否存在

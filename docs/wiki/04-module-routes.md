# routes 模块

**文件**：`src/routes.rs`

## 职责

定义所有 HTTP 路由，挂载中间件，绑定应用状态。

## 路由表

| 方法 | 路径 | Handler | 说明 |
|------|------|---------|------|
| GET | `/` | `health_check` | 健康检查 |
| GET | `/users` | `get_users` | 获取用户列表 |
| POST | `/users` | `create_user` | 创建用户 |
| GET | `/users/{id}` | `get_user` | 获取单个用户 |
| DELETE | `/users/{id}` | `delete_user` | 删除用户 |

## 中间件

`trace_request` 通过 `middleware::from_fn()` 全局挂载，作用于所有路由。

## 注意

- Axum 0.8 路径参数语法为 `{id}`，不是 `:id`
- 状态通过 `.with_state(state)` 注入，必须在中间件 `.layer()` 之后调用

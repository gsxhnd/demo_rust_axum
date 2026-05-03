# API 参考

基础 URL：`http://127.0.0.1:3000`

所有响应头包含 `x-trace-id` 和 `x-span-id`（如有 trace context）。

---

## GET /

健康检查。

**响应**：200 OK

---

## GET /users

获取所有用户。

**响应**：

```json
// 200 OK
[
  {
    "id": 1,
    "name": "张三",
    "email": "zhangsan@example.com",
    "created_at": "2025-01-01T00:00:00"
  }
]
```

**错误**：500 — 数据库查询失败

---

## POST /users

创建用户。

**请求体**：

```json
{
  "name": "张三",
  "email": "zhangsan@example.com"
}
```

**响应**：

```json
// 201 Created
{
  "id": 1,
  "name": "张三",
  "email": "zhangsan@example.com",
  "created_at": "2025-01-01T00:00:00"
}
```

**错误**：500 — 插入失败（如邮箱重复）

---

## GET /users/{id}

获取单个用户。

**路径参数**：`id` — 用户 ID（i32）

**响应**：

```json
// 200 OK
{
  "id": 1,
  "name": "张三",
  "email": "zhangsan@example.com",
  "created_at": "2025-01-01T00:00:00"
}
```

**错误**：
- 404 — 用户不存在
- 500 — 数据库查询失败

---

## DELETE /users/{id}

删除用户。

**路径参数**：`id` — 用户 ID（i32）

**响应**：204 No Content

**错误**：
- 404 — 用户不存在
- 500 — 数据库操作失败

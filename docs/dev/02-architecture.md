# 系统架构

## 整体结构

```
客户端请求
    │
    ▼
┌─────────────────────┐
│   Axum Router       │  routes.rs — 路由定义
│   + trace_request   │  middleware/ — 注入 trace id
│     middleware       │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│   Handlers          │  handlers/ — 业务逻辑
│   (health, user)    │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│   SeaORM            │  models/ — 手写 Entity
│   + PostgreSQL      │
└─────────────────────┘
```

## 启动流程

`main.rs` 按以下顺序执行：

1. `Config::load()` — 加载 `config/{APP_ENV}.toml`，环境变量覆盖
2. `init_tracing()` — 初始化 tracing-subscriber（console / file / json）
3. `init_otel_layer()` — 初始化 OpenTelemetry（如启用）
4. `Database::connect()` — 连接 PostgreSQL（**失败则 panic**）
5. `AppState::new(db)` — 构建应用状态
6. `create_router(state)` — 创建路由并挂载中间件
7. `axum::serve()` — 启动 HTTP 服务

## 请求处理流程

1. 请求进入 → `trace_request` 中间件记录开始时间、生成/提取 trace context
2. 路由匹配 → 分发到对应 handler
3. Handler 通过 `State(state)` 提取 `AppState`，使用 `state.db` 操作数据库
4. 响应返回 → 中间件注入 `x-trace-id` / `x-span-id` 响应头，记录耗时日志

## 状态管理

`AppState` 通过 `.with_state()` 注入路由，包含：

- `db: DatabaseConnection` — SeaORM 数据库连接
- `users: Arc<RwLock<Vec<User>>>` — 内存用户缓存（学习用途）

# 路线图

## 当前状态

项目处于**学习实践阶段**，已实现基础 CRUD 和可观测性框架。

---

## Phase 1 — 基础完善

> 目标：补齐现有功能的短板，让项目达到"可正常开发迭代"的状态。

- [x] **Docker Compose 开发环境**
  - 编排 PostgreSQL + Redis 容器
  - 生成到 `devops/local` 目录
  - 只为本地开发使用，配置直接固定值
- [x] **统一错误处理**
  - handler 全面使用 `Result<T, AppError>` 返回类型
  - 扩展 `AppError` 变体（如 `ValidationError`、`DatabaseError`）
  - 为 SeaORM `DbErr` 实现 `From<DbErr> for AppError`
- [x] **清理 AppState**
  - 移除 `users: Arc<RwLock<Vec<User>>>` 内存列表，完全依赖数据库

## Phase 2 — 功能补全

> 目标：补齐 User CRUD 缺失的操作，加入输入校验和分页，形成完整的 REST 风格 API。

- [ ] **User UPDATE 端点**
  - `PUT /users/{id}` 或 `PATCH /users/{id}`
  - 定义 `UpdateUserRequest` 请求模型
- [ ] **请求参数校验**
  - 引入 `validator` crate 或手写校验逻辑
  - 校验邮箱格式、name 长度等
  - 校验失败返回 `AppError::BadRequest` + 具体字段信息
- [ ] **分页查询**
  - `GET /users?page=1&page_size=20`
  - 定义通用分页请求/响应结构体
  - SeaORM `Paginator` 集成

## Phase 3 — 安全与认证

> 目标：为 API 添加认证保护，引入 Redis 实际使用场景。

- [ ] **认证中间件**
  - JWT 签发与验证（`jsonwebtoken` crate）
  - 登录端点 `POST /auth/login`，返回 token
  - 受保护路由通过中间件提取并校验 token
- [ ] **Redis 集成**
  - 建立 Redis 连接池，注入 `AppState`
  - 用于 token 黑名单 / 会话管理
  - 可选：查询结果缓存、速率限制

## Phase 4 — 质量保障

> 目标：建立测试体系，确保后续迭代不引入回归。

- [ ] **单元测试**
  - 为 `config`、`error`、`models` 模块编写 `#[cfg(test)]` 测试
  - 测试配置加载逻辑（TOML 解析 + 环境变量覆盖）
  - 测试 `AppError` 的 HTTP 状态码映射
- [ ] **集成测试**
  - 使用 `axum::test` 或 `reqwest` 测试 API 端点
  - 测试数据库需独立实例或使用事务回滚
  - 覆盖正常路径 + 错误路径（404、400、500）
- [ ] **CI 集成**
  - `cargo fmt --check` + `cargo clippy -- -D warnings` + `cargo test`
  - GitHub Actions 或其他 CI 平台

## Phase 5 — 文档与可观测性

> 目标：自动生成 API 文档，补充性能基线数据。

- [ ] **API 文档生成**
  - 引入 `utoipa` crate，为 handler 添加 OpenAPI 注解
  - 挂载 Swagger UI 端点（`/swagger-ui`）
  - 确保文档与实际 API 同步
- [ ] **性能基准测试**
  - 使用 `criterion` 或 `wrk` / `hey` 进行负载测试
  - 建立基线数据：QPS、P99 延迟
  - 记录到文档供后续对比

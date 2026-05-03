# 产品范围

## 项目定位

Rust Axum Demo 是一个 **学习与实践项目**，用于探索 Rust 生态中 Web API 服务的开发模式。

## 目标

- 掌握 Axum 0.8 框架的路由、中间件、状态管理等核心概念
- 实践 SeaORM 2.0 的手写 Entity 模式与数据库 CRUD 操作
- 了解 tracing + OpenTelemetry 的可观测性方案
- 建立可复用的 Rust Web 项目脚手架

## 功能边界

### 已实现

- 用户 CRUD（GET / POST / DELETE）
- 健康检查端点
- TOML 配置 + 环境变量覆盖
- 请求追踪中间件（x-trace-id / x-span-id）
- tracing 日志系统（console / file 输出）
- OpenTelemetry 集成（默认关闭）

### 不在范围内

- 认证与授权
- 前端 UI
- 生产级部署方案

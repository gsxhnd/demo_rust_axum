# middleware 模块

**文件**：`src/middleware/mod.rs`

## 职责

提供 HTTP 请求追踪中间件，为每个请求注入 trace context 并记录耗时日志。

## `trace_request` 中间件

### 功能

1. 记录请求开始（method、path、trace context）
2. 将 `RequestTraceContext` 注入请求扩展（`request.extensions_mut()`）
3. 调用 `next.run(request)` 处理请求
4. 根据响应状态码选择日志级别（5xx → error, 4xx → warn, 其他 → info）
5. 在响应头中注入 `x-trace-id` 和 `x-span-id`

### 辅助类型

- `RequestTraceContext` — 持有 `trace_id` 和 `span_id`（均为 `Option<String>`）
- `TRACE_ID_HEADER` / `SPAN_ID_HEADER` — 静态 `HeaderName` 常量

## 注意

代码中使用了 `let ... && let ...` 链式 let 语法（`let_chains`），这是 Rust 2024 Edition 的特性。如果降级 edition 会导致编译失败。

# tracing 模块

**目录**：`src/tracing/`

## 文件结构

- `mod.rs` — 导出公共接口
- `init.rs` — tracing-subscriber 初始化
- `otel.rs` — OpenTelemetry 集成

## 公共接口

| 函数 | 来源 | 说明 |
|------|------|------|
| `init_tracing(&TracingConfig)` | init.rs | 初始化日志系统，返回 `TracingGuards` |
| `init_otel_layer(&TracingConfig, &OpenTelemetryConfig)` | otel.rs | 初始化 OTel layer |
| `shutdown_otel()` | otel.rs | 关闭 OTel，刷新数据 |
| `get_current_trace_id()` | otel.rs | 获取当前 span 的 trace ID |
| `get_current_span_id()` | otel.rs | 获取当前 span ID |
| `format_trace_context()` | otel.rs | 格式化 trace context 字符串 |

## 注意

- **模块命名冲突**：`src/tracing/` 与 `tracing` crate 同名。在此模块内部引用 `tracing` crate 时，需使用 `::tracing::info!()` 或在文件顶部 `use tracing;` 明确引入，避免路径歧义。
- OpenTelemetry 默认关闭（`opentelemetry.enable = false`），启用需在配置中设置 `enable = true` 并提供 OTLP endpoint。
- `TracingGuards` 持有日志 appender 的 guard，必须在 `main` 中保持存活，否则文件日志会丢失。

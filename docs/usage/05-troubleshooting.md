# 故障排查

## 启动失败

### "Database URL not configured"

**原因**：未配置数据库连接。

**解决**：确保 `config/dev.toml` 中有 `[database]` 配置，或设置 `DATABASE_URL` 环境变量。

### "Failed to connect to database"

**原因**：PostgreSQL 不可达或凭据错误。

**排查**：
1. 确认 PostgreSQL 正在运行：`pg_isready`
2. 确认连接字符串正确：`psql "postgresql://user:password@localhost:5432/dev_db"`
3. 确认数据库已创建

### "Failed to bind port 3000"

**原因**：端口被占用。

**解决**：更换端口 `PORT=3001 cargo run`，或终止占用端口的进程。

### 配置文件未加载

**原因**：`APP_ENV=development` 会查找 `config/development.toml`，但项目中文件名为 `config/dev.toml`。

**解决**：使用 `CONFIG_PATH=config/dev.toml cargo run` 或将文件重命名。

## 运行时问题

### 日志未输出到文件

确认 `tracing.output` 包含 `"file"`：

```toml
[tracing]
output = ["console", "file"]
file_path = "logs"
```

### OpenTelemetry 数据未上报

确认配置中 `opentelemetry.enable = true` 且 `otlp_endpoint` 指向可用的 OTLP collector。

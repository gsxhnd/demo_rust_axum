# 快速开始

## 前置条件

- Rust toolchain（nightly 或支持 2024 Edition 的版本）
- PostgreSQL 数据库
- （可选）Redis

## 步骤

### 1. 准备数据库

确保 PostgreSQL 运行中，创建数据库和 `users` 表：

```sql
CREATE DATABASE dev_db;

\c dev_db

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### 2. 配置

编辑 `config/dev.toml`，修改数据库连接信息：

```toml
[database]
url = "postgresql://user:password@localhost:5432/dev_db"
```

或通过环境变量：

```bash
export DATABASE_URL="postgresql://user:password@localhost:5432/dev_db"
```

### 3. 启动

```bash
APP_ENV=development cargo run
```

> 注意：`APP_ENV=development` 会加载 `config/development.toml`。如果你的配置文件是 `config/dev.toml`，请使用 `CONFIG_PATH=config/dev.toml cargo run`。

### 4. 验证

```bash
curl http://127.0.0.1:3000/
```

返回健康检查响应即表示启动成功。

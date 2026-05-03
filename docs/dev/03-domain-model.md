# 领域模型

## User

数据库表名：`users`

| 字段 | 类型 | 说明 |
|------|------|------|
| id | i32 | 主键，自增 |
| name | String(255) | 用户名 |
| email | String(255) | 邮箱，唯一约束 |
| created_at | NaiveDateTime | 创建时间 |

### SeaORM Entity

位于 `src/models/user.rs`，采用**手写模式**（非 `sea-orm-cli` 生成）：

- `Entity` — 实现 `EntityName`，指定表名 `"users"`
- `Model` — 数据结构，派生 `DeriveModel` + `Serialize` / `Deserialize`
- `ActiveModel` — 用于插入/更新操作
- `Column` — 枚举定义列，手动实现 `ColumnTrait::def()`
- `PrimaryKey` — 指定 `Id` 为自增主键

### 请求模型

- `CreateUserRequest` — 创建用户请求体（`name` + `email`），仅派生 `Deserialize`

## 新增模型指南

新增模型时遵循 `models/user.rs` 的手写模式：

1. 在 `src/models/` 下创建新文件
2. 手动定义 `Entity`、`Model`、`Column`、`PrimaryKey`、`Relation`
3. 实现 `ColumnTrait::def()` 指定每列的类型
4. 在 `models/mod.rs` 中导出

# models 模块

**目录**：`src/models/`

## 文件结构

- `mod.rs` — 导出 `User`（Entity 别名）、`ActiveModel`、`CreateUserRequest`、`Relation`
- `user.rs` — User 实体的完整手写定义

## 手写 Entity 模式

本项目**不使用 `sea-orm-cli` 代码生成**，所有 SeaORM Entity 手动编写。

`user.rs` 中手动定义的类型：

| 类型 | 派生宏 | 说明 |
|------|--------|------|
| `Entity` | `DeriveEntity` | 实现 `EntityName`，表名 `"users"` |
| `Model` | `DeriveModel`, `DeriveActiveModel` | 数据结构 + 活动模型 |
| `Column` | `EnumIter`, `DeriveColumn` | 列枚举，手动实现 `ColumnTrait::def()` |
| `PrimaryKey` | `EnumIter`, `DerivePrimaryKey` | 主键定义，`Id` 自增 |
| `Relation` | `EnumIter`, `DeriveRelation` | 关联关系（当前为空） |
| `ActiveModel` | 由 `DeriveActiveModel` 自动生成 | 用于 insert/update |

## 新增模型步骤

1. 创建 `src/models/xxx.rs`，按 `user.rs` 模式定义所有类型
2. 在 `src/models/mod.rs` 中添加 `pub mod xxx;` 和 `pub use` 导出
3. 手动实现 `ColumnTrait::def()` 指定每列的数据库类型

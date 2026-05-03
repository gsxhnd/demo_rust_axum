# 安装部署

## 开发构建

```bash
cargo build
```

## 生产构建

```bash
cargo build --release
```

产物位于 `target/release/rust_axum`。

## 运行

```bash
# 开发
APP_ENV=development cargo run

# 生产（使用编译后的二进制）
APP_ENV=production ./target/release/rust_axum
```

## 代码检查

```bash
cargo fmt            # 格式化
cargo clippy -- -D warnings  # Lint 检查
cargo test           # 运行测试
```

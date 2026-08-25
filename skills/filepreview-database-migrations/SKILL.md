---
name: filepreview-database-migrations
description: FilePreview SQLx/SQLite 数据库迁移规范。修改 SQLite 表、字段、索引、约束或已有数据时使用。
---

# FilePreview 数据库迁移规范

## 规则

- 新增、删除或重命名表和字段，以及修改类型、约束、索引或需要数据回填时，必须新增 `src-tauri/migrations/<编号>_<说明>.sql`。
- 已发布 migration 不得修改。SQLx 会校验 migration 校验和。
- migration 必须使用 LF 换行；`.gitattributes` 已固定 `src-tauri/migrations/*.sql` 为 `eol=lf`。
- Rust 启动时使用 `sqlx::migrate!` 执行 migration；不得以运行时代码 `ALTER TABLE` 或隐式建表代替 migration。
- 用户数据库固定为用户数据目录中的 `filepreview.db`，禁止使用仓库内数据库替代。

## 工作流程

1. 查看现有 migration 并确认下一个递增编号。
2. 同步更新 model、DAO、service 和 Tauri Command 类型。
3. 在临时 SQLite 数据库上验证从空库迁移到最新版本。
4. 对有历史数据的变更明确回填、兼容或备份策略。
5. 运行 Rust 格式、编译和测试检查。

## 验证

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --lib
```

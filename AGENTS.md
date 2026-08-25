# FilePreview 项目规范

本文件适用于整个 FilePreview 仓库。项目使用 Rust、Tauri 2、Vue 3 和 SQLite。

进行代码修改前，按改动范围阅读对应 skill：

- 修改 `src/` 下的 Vue 页面、组件、Pinia store、API 客户端或样式时，阅读 [`skills/filepreview-frontend/SKILL.md`](skills/filepreview-frontend/SKILL.md)。
- 修改 `src-tauri/src/` 下的 command、service、dao、model、数据库访问或文件系统业务时，阅读 [`skills/filepreview-backend/SKILL.md`](skills/filepreview-backend/SKILL.md)。
- 修改 SQLx migration、SQLite 表、字段、索引、约束或已有数据时，额外阅读 [`skills/filepreview-database-migrations/SKILL.md`](skills/filepreview-database-migrations/SKILL.md)，并同步维护 `src-tauri/migrations/`。
- 执行 Windows 打包、安装包生成、版本升级或发布验证时，阅读 [`skills/filepreview-windows-release/SKILL.md`](skills/filepreview-windows-release/SKILL.md)。
- 配置、触发或排查 GitHub Actions 构建时，阅读 [`skills/filepreview-github-actions/SKILL.md`](skills/filepreview-github-actions/SKILL.md)。

## 通用要求

- 前端保持 `api -> store -> page/component` 分层；后端保持 `commands -> service -> dao` 分层。
- 数据库固定使用用户数据目录中的 `filepreview.db`。表结构变更必须追加 SQLx migration，禁止在启动时临时 `ALTER TABLE`。
- Rust 代码使用 `cargo fmt`；前端使用项目 Prettier 配置。
- 修改后按影响范围运行检查；涉及前后端契约、应用生命周期或数据库时，运行：

  ```powershell
  npm run format:check
  npm run build
  cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  cargo check --manifest-path src-tauri/Cargo.toml
  cargo test --manifest-path src-tauri/Cargo.toml --lib
  ```

- 默认保留工作区改动，不自动提交或推送；只有用户明确要求时才创建中文说明的本地提交或推送远程。

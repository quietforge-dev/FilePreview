---
name: filepreview-backend
description: FilePreview Rust/Tauri 后端开发规范。修改 src-tauri/src 下的 command、service、dao、model、数据库或文件系统业务时使用。
---

# FilePreview 后端规范

## 分层

- `commands/` 负责 Tauri Command 的参数与返回值；`service/` 负责工作区、历史和预览业务；`dao/` 只负责 SQLx 查询和写入；`model/` 定义持久化和 IPC 类型。
- 本地文件操作通过 Tauri IPC 完成，不为本地功能引入 HTTP 中转。
- 数据库访问必须通过 DAO，页面和 Command 不直接写 SQL。

## 文件系统与数据库

- 所有读取路径必须 canonicalize 后确认位于当前工作区内；符号链接不能绕过工作区边界。
- 目录和文件读取失败返回可读错误，不应导致桌面应用退出。
- 数据库存放在 `ProjectDirs` 对应的用户数据目录中，文件名为 `filepreview.db`。
- 新表或字段必须通过 `src-tauri/migrations/` 追加 SQLx migration，禁止启动时临时修改表结构。

## 并发与错误

- 使用项目 `AppError` 和 `Result`，避免 `unwrap` 处理外部输入、文件系统和数据库错误。
- SQL 使用参数绑定；列表排序必须有稳定兜底键，查询限制返回数量。
- 异步路径避免阻塞式大文件或目录扫描；需要扩展时优先使用 Tokio 异步 API 或 `spawn_blocking`。

## 验证

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --lib
```

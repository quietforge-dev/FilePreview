---
name: filepreview-github-actions
description: FilePreview GitHub Actions 构建规范。配置、触发、查看或排查 Windows/macOS Tauri 构建时使用。
---

# FilePreview GitHub Actions

- 工作流应使用 Node.js、Rust stable、`npm ci`、`npm run build` 和 Tauri CLI 构建桌面应用。
- Windows 与 macOS 构建分别产出当前架构的安装包或应用包；不上传用户数据目录中的 `filepreview.db`、日志或配置。
- 构建前确认 `src-tauri/migrations/`、图标和前端 `dist` 会随应用打包。
- 排查顺序：查看 runner 架构和工具链，再检查 `npm ci`、前端构建、Cargo/Tauri 构建及产物路径。
- 通过浏览器操作 GitHub Actions 前确认已登录；遇到登录页时停止并提示用户登录。
- 不自动推送、创建 tag、发布 Release 或上传外部产物，除非用户明确要求。

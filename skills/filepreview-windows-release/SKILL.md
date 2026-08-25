---
name: filepreview-windows-release
description: FilePreview Windows/Tauri 打包、版本升级和发布验证规范。用户要求构建安装包、升级版本或发版时使用。
---

# FilePreview Windows 发版

1. 检查 Git 状态、当前版本以及 Node 和 Cargo 工具链。
2. 版本升级时同步更新 `package.json`、`package-lock.json`、`src-tauri/Cargo.toml`、`src-tauri/Cargo.lock` 和 `src-tauri/tauri.conf.json`。
3. 打包前运行前端构建、Rust 检查以及 `npm run tauri build`。
4. 检查安装包中的图标、前端产物和 `src-tauri/migrations/`，并确认启动后数据库位于用户数据目录而非安装目录。
5. 不自动提交、推送、创建 tag、上传产物或安装验证，除非用户明确要求。

数据库结构变化额外遵守 `../filepreview-database-migrations/SKILL.md`。

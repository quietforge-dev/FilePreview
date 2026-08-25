# FilePreview

FilePreview 是一个本地优先的桌面文件浏览与预览工具，使用 Rust、Tauri 2、Vue 3 和 SQLite 构建。

当前支持 Markdown、Word、PowerPoint 和文本文件预览；不支持的格式会明确提示。

## 开发

```powershell
npm install
npm run dev:desktop
```

历史数据保存在操作系统用户数据目录中的 `filepreview.db`，不会写入安装目录。

## 发布

推送 `v*` tag 会触发 GitHub Actions 打包并创建 Release。首次配置自动更新前，请阅读 [应用内更新发布说明](docs/应用内更新发布说明.md)。

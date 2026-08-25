# FilePreview

FilePreview 是一个本地优先的桌面文件浏览与预览工具，使用 Rust、Tauri 2、Vue 3 和 SQLite 构建。

当前支持 Markdown、文本、图片和 PDF 预览。安装 LibreOffice 后，还支持通过高保真 PDF 转换预览 `.doc`、`.docx`、`.ppt`、`.pptx`、`.xls` 和 `.xlsx`；不支持的格式会明确提示。

首次打开 Office 文件时，如未检测到 LibreOffice，预览区会提供 Windows 快速安装和官方下载安装入口。文件始终在本地转换和预览，不会上传或修改原文件。

## 开发

```powershell
npm install
npm run dev:desktop
```

历史数据保存在操作系统用户数据目录中的 `filepreview.db`，不会写入安装目录。

## 发布

推送 `v*` tag 会触发 GitHub Actions 打包并创建 Release。首次配置自动更新前，请阅读 [应用内更新发布说明](docs/应用内更新发布说明.md)。

Windows 本地发布构建使用 `scripts/build_release.bat`。该入口脚本读取被 Git 忽略的 `src-tauri/keys/updater.key`，生成并校验已签名的 NSIS 安装包和更新签名文件。

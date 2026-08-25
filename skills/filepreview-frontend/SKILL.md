---
name: filepreview-frontend
description: FilePreview Vue 3/Tauri 前端开发规范。修改 src 下的页面、组件、Pinia store、API 客户端或样式时使用。
---

# FilePreview 前端规范

## 分层

- 使用 Vue 3 Composition API、TypeScript、Vite、Element Plus、Pinia 和 SCSS。
- `src/api/` 封装 Tauri Command 调用及类型；`src/stores/` 管理跨组件状态；`src/pages/` 负责页面布局与编排；`src/components/` 提供可复用交互。
- 页面不直接访问 Rust 状态或文件系统，必须经过 API 和 store。

## 交互

- 打开工作区、切换目录、刷新目录和选择历史记录后，刷新受影响的列表或菜单数据。
- 加载、空数据、错误和保存状态必须可见；历史记录写入失败不能阻止文件预览。
- 图标按钮应使用 Element Plus Icons 并提供 `aria-label` 或 tooltip。
- 保持三栏桌面布局紧凑、可扫描，最小宽度下不得遮挡文件名或预览内容。

## 预览器

- 格式判断只允许在 `PreviewRenderer` 的 `canHandle` 中实现。
- 新增格式时注册渲染器，不在页面或文件列表中堆叠扩展名分支。
- 通过 `v-html` 显示的 Markdown 或 Office HTML 必须经过清理。
- Blob URL 在切换图片预览时释放。

## 验证

```powershell
npm run format:check
npm run build
```

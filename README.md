# FilePreview

FilePreview 是一个本地优先的桌面文件浏览与预览工具。它适合在项目目录、资料目录或下载目录中快速定位文件、保持多个预览上下文，并在不离开当前工作区的情况下阅读常见文档内容。

当前版本：`0.0.6`

FilePreview 使用 Rust、Tauri 2、Vue 3 和 SQLite 构建，重点是本机文件的快速浏览：目录树按需加载、标签页保留工作区与文件、文件系统变更自动刷新，并支持文件内容搜索及 Office 文档预览。

## 功能亮点

- 本地优先：文件在本机读取、转换和预览，不上传文件内容。
- 目录树：默认加载一层目录；展开文件夹时再读取子项，适合大型目录。
- 标签页：可并行保留多个工作区和预览文件，支持恢复上次会话。
- 文件监听：当前工作区发生文件新增、修改、移动或删除时自动合并刷新。
- 内容搜索：搜索文本、代码和 Markdown 文件中的内容。
- 多格式预览：为 Markdown、文本代码、图片、PDF 和 Office 文件提供对应预览器。
- Markdown 编辑：`.md` 和 `.markdown` 默认预览，可切换源码编辑、保存及处理外部修改冲突。
- 文件操作：支持复制、粘贴、复制路径、使用系统默认程序打开、在系统文件管理器中显示，以及移入系统回收站。

## 支持格式

| 类别 | 格式 | 预览方式 |
| --- | --- | --- |
| Markdown | `.md`、`.markdown` | 渲染预览；支持源码编辑和保存 |
| 文本与代码 | 常见文本、配置与代码文件 | 文本预览 |
| 图片 | 常见图片格式 | 图片预览 |
| PDF | `.pdf` | PDF 预览 |
| Word | `.doc`、`.docx` | LibreOffice 转换为 PDF 后预览 |
| PowerPoint | `.ppt`、`.pptx` | LibreOffice 转换为 PDF 后预览 |
| Excel | `.xls`、`.xlsx` | LibreOffice 转换为 PDF 后预览 |

不支持的格式会在预览区明确提示，不会尝试上传或转换到远程服务。

## Office 与 LibreOffice

FilePreview 不随安装包分发 LibreOffice。Office 文档通过本机 LibreOffice 转换为 PDF 后预览，以保持文档版式并避免将文件发送到服务器。

- 未安装 LibreOffice 时，应用内会提供 Windows 快速安装或跳转官方下载安装页的入口。
- Office 文件需要先转换，预览速度可能慢于文本、Markdown 和图片。
- 转换不会修改原始文件；源文件和转换过程均保留在本机。
- `.doc`、`.ppt`、`.xls` 等旧版 Office 格式同样依赖 LibreOffice。

## 隐私与本地数据

- 文件默认只在本机读取、转换和预览，不上传文件内容。
- 应用的 SQLite 数据库为 `filepreview.db`，保存在操作系统用户数据目录。
- 历史记录、标签页和界面设置写入该用户数据目录，不写入应用安装目录。
- 删除功能优先调用系统回收站；仍请在删除前确认目标和目录内容。

## 快速开始

从源码启动开发环境：

```powershell
npm install
npm run dev:desktop
```

启动后选择一个本地文件夹，即可浏览目录树和预览文件。

## 开发环境

项目使用 Node.js、Rust stable 工具链，以及 Tauri 2 对应的 Windows 构建环境。

```powershell
npm install
npm run dev:desktop
npm run build
```

前端构建校验：

```powershell
npm run format:check
npm run build
```

后端构建校验：

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --lib
```

## 项目架构

```text
src/
  api/          Tauri command 与系统插件调用封装
  stores/       工作区、标签页、预览、编辑会话和设置状态
  pages/        页面布局与业务编排
  components/   目录树、标签栏、右键菜单和各类预览组件

src-tauri/src/
  commands/     Tauri IPC command
  service/      工作区、预览、监听、历史与 Office 业务
  dao/          SQLite 查询与写入
  filesystem.rs 受工作区边界保护的本地文件操作
```

前端遵循 `api -> store -> page/component` 分层；后端遵循 `commands -> service -> dao/filesystem` 分层。Markdown 由渲染器和编辑器分别负责预览与源码编辑；PDF、图片、文本和 Office 使用对应预览器，其中 Office 预览服务负责调用 LibreOffice 转换。工作区服务管理路径边界，文件监听服务向前端发送合并后的变更事件，标签页和应用设置通过 SQLite 持久化。

## 构建与发布

Windows 本地发布构建使用：

```powershell
scripts\build_release.bat
```

该脚本读取被 Git 忽略的 `src-tauri/keys/updater.key`，生成并校验已签名的 NSIS 安装包及更新签名文件。推送符合 `v*` 形式的 tag 会触发 GitHub Actions 构建并创建 Release。发布自动更新前，请阅读 [应用内更新发布说明](docs/应用内更新发布说明.md)。

## 当前状态与路线图

`0.0.6` 已具备本地目录浏览、标签页、文件监听、文本内容搜索、Markdown 编辑、PDF 和 Office 预览等能力。

后续方向包括更丰富的图片查看控制、深色主题、多标签页交互优化、文件编辑能力扩展及远程文件系统支持。具体排期以项目 issue 和发布计划为准。

## 贡献

欢迎通过 issue 提交可复现的问题、功能建议或文档改进。提交代码前请保持前后端分层，并运行与改动范围匹配的格式化、构建和测试命令。

## 许可证

本项目采用仓库中的 [LICENSE](LICENSE) 许可证。

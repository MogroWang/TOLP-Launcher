# TOLP Launcher · 光点之旅启动器

《光点之旅（Tour of Light Point）》的桌面启动器：启动本地目录下的网页版游戏，并提供启动方式（全屏 / 窗口化）等设置。

技术栈：**Vue 3 + TypeScript + Vite**（前端）/ **Tauri 2 + Rust**（后端）。

## 工作方式

- 启动器内嵌一个仅监听 `127.0.0.1` 随机端口的静态文件服务器（Rust / tiny_http）来提供游戏文件——GDevelop 网页版导出必须经 HTTP 访问。
- 点击「开始游戏」后，游戏以内嵌方式加载进主窗口（iframe，仅指向本地服务器、无任何 IPC 权限），右上角提供「退出游戏」胶囊返回主页。
- 「全屏启动」= 主窗口进入无边框全屏再进入游戏；「窗口化启动」= 保持普通窗口直接开始。游戏地址使用 `localhost` 而非回环 IP，避免被系统代理规则拦截。
- 游戏文件地址一律来自本机，服务器随应用生命周期存活，重复点击「开始游戏」会复用同一服务器。

## 游戏文件放置

启动器按以下顺序查找游戏目录：

1. 「启动设置 → 游戏目录」中指定的目录；
2. 启动器 exe 同目录的 `game/` 文件夹（便携版默认约定）；
3. 开发构建（`tauri dev`）下回退到仓库根目录的 `game/`。

要求目录中直接包含 GDevelop「网页版 / HTML5」导出的 `index.html`。也可以用 `dev/sample-game/` 里的测试页验证整条链路：在启动设置中选择该目录后点击「开始游戏」。

设置保存在 exe 同目录的 `launcher-settings.json`（便携约定，不写注册表 / AppData）。

## 本地开发

要求：Node.js ≥ 20、Rust stable（MSVC）、WebView2 Runtime（Win10/11 一般自带）。

```bash
npm install
npm run tauri dev      # 开发调试
npm run tauri -- build --no-bundle   # 构建便携版裸 exe
```

构建产物：`src-tauri/target/release/TOLP-Launcher.exe`（或 `target/<target>/release/` 下）。把它和 `game/` 文件夹放在一起即可分发。

## 资源再生成

- 图标（紫色光点 + 播放键，源自游戏 UI 设计）：`npm run icons` → `src-tauri/icons/`
- 字体子集（游戏项目「未来圆」字体，仅保留界面字符）：`node scripts/subset-fonts.mjs "<未来圆系列字体目录>"` → `src/assets/fonts/`

两者产物均已提交入库，日常构建无需重新生成。**发布前请自行确认字体授权允许随应用分发。**

## CI

`.github/workflows/build-windows.yml`：在 `windows-latest` 上构建 **Windows x64 便携版**（裸 exe + 说明文件打包为 zip，不打安装包）。触发方式：推送 `v*` 标签（会同时创建 GitHub Release 并附上压缩包）或在 Actions 页面手动触发（workflow_dispatch）。

## 目录结构

```
├─ src/                  # Vue 前端
│  ├─ components/        # 标题栏 / 主视觉光点 / 启动设置面板 / 游戏舞台（iframe）
│  └─ lib/tauri.ts       # 后端命令封装
├─ src-tauri/            # Tauri + Rust 后端
│  └─ src/
│     ├─ game_server.rs  # 本地静态服务器
│     ├─ launcher.rs     # 游戏目录解析与窗口启动
│     └─ settings.rs     # 设置读写（exe 同目录 JSON）
├─ game/                 # 游戏文件放置处（README.txt 入库，游戏本体不入库）
├─ dev/sample-game/      # 开发用测试页
├─ scripts/              # 图标 / 字体子集生成脚本
└─ .github/workflows/    # CI
```

## UI 设计说明

视觉语言来自游戏 4.0 的 UI 设计稿：纯黑背景、发光球体（紫色 = 主操作、白色 = 次级操作）、白色圆角图标与克制的单一紫色强调色；交互动效遵循即时按压反馈、可打断过渡与 `prefers-reduced-motion` 降级。

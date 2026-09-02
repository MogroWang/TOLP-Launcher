# TOLP Launcher · 光点之旅启动器

《光点之旅（Tour of Light Point）》的桌面启动器：启动本地目录下的网页版游戏，并提供游戏版本管理、启动方式（全屏 / 窗口化）等设置。

技术栈：**Vue 3 + TypeScript + Vite**（前端）/ **Tauri 2 + Rust**（后端）。

## 工作方式

- 启动器内嵌一个仅监听 `127.0.0.1` 随机端口的静态文件服务器（Rust / tiny_http）来提供游戏文件——GDevelop 网页版导出必须经 HTTP 访问。
- 点击「开始游戏」会按启动设置**新开一个独立游戏窗口**：全屏（无边框）或 1280×720 窗口；关闭游戏窗口即退出游戏，启动器不受影响。重复点击会复用已有游戏窗口（运行中点击可召回窗口）。
- 主界面实时显示游戏运行状态：游戏窗口启动后显示「正在运行」，游戏关闭后自动恢复「开始游戏」。
- 游戏地址使用 `localhost` 而非回环 IP，避免被系统代理规则拦截；游戏窗口不拥有任何 IPC 权限，与启动器相互隔离。
- 启动命令为 async command：窗口创建需要与主线程事件循环交互，同步 command 会与其互锁导致应用挂起。

## 游戏文件放置

启动设置中可先选择「游戏版本」：

- **内置版本**（当前为占位符 `1.0.0`）：优先使用启动器 exe 同目录 `games/<版本>/` 托管目录，版本尚未提供下载前回落到同目录 `game/` 文件夹；
- **自定义目录**：按以下顺序查找游戏目录：

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

`.github/workflows/ci.yml`：在 `windows-latest` 上构建 **Windows x64 便携版**（裸 exe + 说明文件打包为 zip，不打安装包）。触发方式：

- 推送到 `main` 分支或提交 Pull Request：自动构建并上传构建产物（验证用）；
- 推送 `v*` 标签：构建并创建 GitHub Release，附上便携版压缩包；
- Actions 页面手动触发（workflow_dispatch）。

发布流程：修改版本号（`package.json` / `src-tauri/tauri.conf.json` / `src-tauri/Cargo.toml`）→ 更新 `CHANGELOG.md` → 提交并打 `v0.x.0` 标签推送，CI 自动完成构建与发布。

## 目录结构

```
├─ src/                  # Vue 前端
│  ├─ components/        # 标题栏 / 主视觉光点 / 启动设置抽屉
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

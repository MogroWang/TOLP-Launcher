## TOLP Launcher 0.5.1 修复计划(不进行本地构建)

已通读全部核心代码(Rust 后端 5 个文件、前端 10 个组件 + 2 个 lib、配置与 CI)。以下为确认的问题与修复方案。

### 一、安全修复(后端)

**1. `src-tauri/src/game_server.rs` — 本地静态服务器目录穿越漏洞(P0)**
`sanitize_path` 只按 `/` 切分并拒绝 `..`,存在两个 Windows 下的绕过:
- URL 中 `%5C`(反斜杠)解码后不被视为分隔符,`..\..\xxx` 作为一个整体组件通过检查,而 Windows 文件系统解析路径时 `\..\` 依然指向上级目录;
- 更严重:组件形如 `C:\Windows\...` 时,`PathBuf::push` 因该参数是"绝对路径"会**整体替换**根目录,监听在 127.0.0.1 的服务器可被用来读取任意盘符文件。

修复:解码后按 `['/', '\\']` 双分隔符切分(反斜杠视作分隔符,顺带使 `%5C..%5C` 形式也命中 `..` 拒绝),并拒绝包含 `:` 的组件(防盘符路径 / NTFS 备用数据流)。

### 二、崩溃修复(前端)

**2. `src/lib/i18n.ts` — 非法语言值导致整页白屏(P0)**
`setLocale` 不校验参数,`t()` 直接 `messages[locale.value][key]`。设置文件中 `language` 字段若被手改为非法值,启动即抛 TypeError 白屏,且无法从界面恢复。修复(双保险):
- `setLocale` 校验语言值,非法时回落 `zh-CN`;
- `t()` 改为防御式查找 `messages[locale.value]?.[key] ?? messages['zh-CN'][key] ?? key`。

### 三、功能 bug 修复

**3. `src/App.vue` — listVersions 失败时误重置版本(P1)**
`onMounted` 中 `listVersions().catch(() => null)` 失败时 `knownIds` 只含内置版本,数据文件夹中的合法版本 id 会被误判无效并重置为内置版本、写回设置文件。修复:仅在 `scan` 非 null 时执行归一化重置,扫描失败时保留原值。

**4. `src/components/QuickLaunch.vue` — 版本下拉框误切换(P1)**
当前选中数据文件夹版本(如 `2.0.0`)时,下拉框第一项 label 显示 `2.0.0`,但点击会执行 `setVersion('builtin')` 把 `versionId` 设为内置 `4.0.002`,所见与所得不符。修复:`setVersion('builtin')` 时若当前已是任意版本启动(`versionId !== null`)则不提交,第一项语义为"保持当前所选版本";同时修正该场景下的描述文案(不再显示"内部开发版本",在 i18n 新增一条通用描述 key)。

**5. `src-tauri/src/launcher.rs` + `game_server.rs` — 复用游戏窗口时不刷新已变更的目录(P1)**
`launch_game` 对已存在的游戏窗口只 show/focus,而 `set_root` 已把服务器指向新目录;状态失同步时点「启动游戏」,窗口仍是旧版本内容。修复:`GameServer` 增加当前 root 的只读访问;`launch_game` 中若 root 发生变化且复用已有窗口,调用 `navigate(url)` 重新加载到新版本(root 未变则不干扰)。

**6. `src/lib/tauri.ts` — launchGame 缺少非 Tauri 环境降级(P2)**
其它 API 均有 `!inTauri` 降级,唯独 `launchGame` 直接 invoke,浏览器预览下点击会把原始错误抛到界面。修复:非 Tauri 环境 reject `请在桌面应用中运行`(与已有 i18n 文案一致)。

**7. `src/components/AboutPage.vue` — getDataDir 缺错误兜底(P2)**
`await getDataDir()` 无 catch,Tauri 下命令失败(极罕见)会产生 unhandled rejection。修复:`.catch(() => '')`,界面回落显示 `—`。

### 四、版本升级 0.5.0 → 0.5.1

同步修改以下 4 处版本号:
- `package.json`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`(`[[package]] name = "tolp-launcher"` 块,手动文本编辑,不运行 cargo)

并更新 `CHANGELOG.md`:按 Keep a Changelog 格式新增 `[0.5.1]` 条目(修复类:上述 1–7),底部补 `[0.5.1]: ../../releases/tag/v0.5.1` 链接。

### 五、检查过但不修改的项(说明理由)

- **后端窗口尺寸 clamp 下限 320×240 比前端滑块 640×360 宽松**:宽容方向的一致性差异,收紧反而可能影响手改过配置的用户,不动。
- **CSP 为 null**:全本地资源 + 游戏窗口本就加载外部 http,保持现状。
- **主窗口关闭后游戏窗口独立存续**:0.1.0 起的有意设计(「游戏窗口与启动器相互隔离」),不动。
- **静态服务器不支持 Range 请求**:0.1.0 起行为,GDevelop 游戏为整载资源,不动。
- **快速启动下拉框只列「内置版本 / 自定义启动」而不列全部数据文件夹版本**:属功能增强而非 bug,0.5.1 只做防误切的最小修复,完整版本列表留待后续版本。

### 六、验证方式

- 前端改动:运行 `npm run typecheck`(vue-tsc --noEmit,纯类型检查,无构建产物,不属于"本地构建")。
- Rust 改动:不运行任何 cargo 命令,靠逐行人工复查(改动集中在 `sanitize_path` 与 `launch_game` 复用分支,逻辑简单)。
- 完成后汇报修改清单;默认不提交 git,如需提交/打 `v0.5.1` 标签请告知。
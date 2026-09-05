//! 游戏目录解析、版本管理与游戏启动（独立游戏窗口）。

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::data_dir;
use crate::game_server::GameServer;
use crate::settings::{self, LaunchMode, Settings};

pub struct LauncherState {
    pub server: Option<GameServer>,
    /// 游戏窗口是否正在运行（前端据此在「启动游戏 / 取消运行」间切换）
    pub game_running: bool,
}

const GAME_WINDOW_LABEL: &str = "game";
const DEFAULT_WINDOWED_SIZE: (f64, f64) = (1280.0, 720.0);
/// 窗口大小的安全夹取范围，防止设置文件被手改成荒谬值
const MIN_WINDOW_SIZE: (f64, f64) = (320.0, 240.0);
const MAX_WINDOW_SIZE: (f64, f64) = (7680.0, 4320.0);

/// 窗口化启动的窗口大小：取设置值并夹取到安全范围，未设置时用默认 1280×720。
fn windowed_size(settings: &Settings) -> (f64, f64) {
    settings
        .windowed_size
        .map(|s| {
            (
                (s.width as f64).clamp(MIN_WINDOW_SIZE.0, MAX_WINDOW_SIZE.0),
                (s.height as f64).clamp(MIN_WINDOW_SIZE.1, MAX_WINDOW_SIZE.1),
            )
        })
        .unwrap_or(DEFAULT_WINDOWED_SIZE)
}

/// 游戏窗口销毁后通知前端恢复「启动游戏」状态。
pub const GAME_CLOSED_EVENT: &str = "game-closed";

/// 内置游戏版本 id：优先从数据文件夹 `versions/4.0.002/` 识别。
pub const BUILTIN_VERSION_ID: &str = "4.0.002";

/// 官方游戏 id：manifest.webmanifest 的 `id` 字段与之一致时视为官方版本。
pub const OFFICIAL_GAME_ID: &str = "com.mws.tolp";

fn has_index_html(dir: &Path) -> bool {
    dir.join("index.html").is_file()
}

/// 数据文件夹托管目录：`launcher-data/versions/<id>/`。
fn managed_version_dir(version_id: &str) -> Option<PathBuf> {
    let dir = data_dir::versions_dir().ok()?.join(version_id);
    has_index_html(&dir).then_some(dir)
}

/// 内置版本的自定义位置（用户在版本管理中指定）。
fn custom_version_dir(settings: &Settings) -> Option<PathBuf> {
    let dir = PathBuf::from(settings.custom_version_dir.as_deref()?.trim());
    (!dir.as_os_str().is_empty() && has_index_html(&dir)).then_some(dir)
}

/// 0.3.0 旧托管目录：exe 同目录 `games/<id>/`（向后兼容保留识别）。
fn legacy_version_dir(version_id: &str) -> Option<PathBuf> {
    let dir = std::env::current_exe()
        .ok()?
        .parent()?
        .join("games")
        .join(version_id);
    has_index_html(&dir).then_some(dir)
}

/// 便携版默认游戏目录：exe 同目录 `game/`。
fn portable_game_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|parent| parent.join("game")))
}

/// 开发构建下回退到工程根目录的 `game`（便于 `tauri dev` 调试）。
#[cfg(debug_assertions)]
fn dev_game_dir() -> Option<PathBuf> {
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("game");
    has_index_html(&dev).then_some(dev)
}

#[cfg(not(debug_assertions))]
fn dev_game_dir() -> Option<PathBuf> {
    None
}

/// 解析当前生效的游戏目录：
/// 1. 选中版本时：优先使用数据文件夹 `versions/<版本>/`；内置版本再依次尝试
///    用户指定的自定义位置、旧版 `games/<版本>/` 托管目录；
/// 2. 自定义启动（未选版本）时：设置中指定的目录；
/// 3. 可执行文件同目录的 `game` 文件夹（便携版默认约定）；
/// 4. 开发构建下回退到工程根目录的 `game`。
pub fn resolve_game_dir(settings: &Settings) -> Option<PathBuf> {
    if let Some(version_id) = settings.version_id.as_deref() {
        if let Some(dir) = managed_version_dir(version_id) {
            return Some(dir);
        }
        if version_id == BUILTIN_VERSION_ID {
            if let Some(dir) = custom_version_dir(settings).or_else(|| legacy_version_dir(version_id)) {
                return Some(dir);
            }
        }
    }

    if let Some(dir) = settings.game_dir.as_deref() {
        if !dir.trim().is_empty() {
            return Some(PathBuf::from(dir));
        }
    }
    portable_game_dir().or_else(dev_game_dir)
}

// ---------- manifest 识别 ----------

/// 游戏目录下 manifest.webmanifest 识别到的信息。
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestInfo {
    /// manifest 的 `id` 字段（官方为 com.mws.tolp）
    pub id: Option<String>,
    /// manifest 的 `name`（缺失时回落 short_name）
    pub name: Option<String>,
    /// manifest 的 `version` 字段
    pub version: Option<String>,
    /// `id` 与官方 id 一致
    pub official: bool,
}

/// 读取游戏目录下 manifest.webmanifest 的版本信息；缺失或损坏时返回空信息。
pub fn read_manifest(dir: &Path) -> ManifestInfo {
    let mut info = ManifestInfo::default();
    let Ok(text) = fs::read_to_string(dir.join("manifest.webmanifest")) else {
        return info;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return info;
    };
    let field = |key: &str| {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };
    info.id = field("id");
    info.name = field("name").or_else(|| field("short_name"));
    info.version = field("version");
    info.official = info.id.as_deref() == Some(OFFICIAL_GAME_ID);
    info
}

// ---------- 状态与命令 ----------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStatus {
    pub found: bool,
    pub dir: Option<String>,
    pub reason: Option<String>,
    /// manifest.webmanifest 识别到的游戏版本号
    pub version: Option<String>,
    /// manifest id 是否为官方 com.mws.tolp
    pub official: bool,
}

#[tauri::command]
pub fn game_status() -> GameStatus {
    let settings = settings::load();
    let resolved = resolve_game_dir(&settings);
    let mut status = match &resolved {
        Some(dir) if has_index_html(dir) => GameStatus {
            found: true,
            dir: Some(dir.display().to_string()),
            reason: None,
            version: None,
            official: false,
        },
        Some(dir) if dir.exists() => GameStatus {
            found: false,
            dir: Some(dir.display().to_string()),
            reason: Some("游戏目录中缺少 index.html".to_string()),
            version: None,
            official: false,
        },
        Some(dir) => GameStatus {
            found: false,
            dir: Some(dir.display().to_string()),
            reason: Some("尚未找到游戏文件夹".to_string()),
            version: None,
            official: false,
        },
        None => GameStatus {
            found: false,
            dir: None,
            reason: Some("尚未找到游戏文件夹".to_string()),
            version: None,
            official: false,
        },
    };
    if status.found {
        if let Some(dir) = &resolved {
            let manifest = read_manifest(dir);
            status.version = manifest.version;
            status.official = manifest.official;
        }
    }
    status
}

/// 版本条目：数据文件夹版本目录中识别到的一个版本，或自定义位置。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionEntry {
    /// 版本 id：版本文件夹下的子目录名
    pub id: String,
    pub dir: String,
    /// 目录中存在 index.html（可直接启动）
    pub found: bool,
    pub reason: Option<String>,
    /// manifest.webmanifest 的 name
    pub name: Option<String>,
    /// manifest.webmanifest 的 version
    pub version: Option<String>,
    /// manifest id 为官方 com.mws.tolp
    pub official: bool,
}

impl VersionEntry {
    fn from_dir(id: String, dir: &Path) -> Self {
        let found = has_index_html(dir);
        let manifest = read_manifest(dir);
        Self {
            id,
            dir: dir.display().to_string(),
            found,
            reason: (!found).then(|| "游戏目录中缺少 index.html".to_string()),
            name: manifest.name,
            version: manifest.version,
            official: manifest.official,
        }
    }
}

/// 版本扫描结果：供版本管理页展示。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionScan {
    /// 启动器数据文件夹根目录
    pub data_dir: String,
    /// 版本文件夹路径（versions/）
    pub versions_dir: String,
    /// versions/ 下自动识别的版本（含未就绪目录，便于提示缺 index.html）
    pub versions: Vec<VersionEntry>,
    /// 用户指定的自定义位置（内置版本的备用目录）
    pub custom: Option<VersionEntry>,
}

/// 扫描数据文件夹 versions/ 中的版本；未识别到时由前端引导用户指定自定义位置。
#[tauri::command]
pub fn list_versions() -> VersionScan {
    let settings = settings::load();
    let versions_dir = data_dir::versions_dir().ok();
    let mut versions = Vec::new();
    if let Some(vd) = &versions_dir {
        if let Ok(entries) = fs::read_dir(vd) {
            let mut dirs: Vec<_> = entries.flatten().filter(|e| e.path().is_dir()).collect();
            dirs.sort_by_key(|e| e.file_name());
            for entry in dirs {
                let id = entry.file_name().to_string_lossy().to_string();
                if id.starts_with('.') {
                    continue;
                }
                versions.push(VersionEntry::from_dir(id, &entry.path()));
            }
        }
    }
    let custom = settings
        .custom_version_dir
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|dir| VersionEntry::from_dir(BUILTIN_VERSION_ID.to_string(), Path::new(dir)));
    VersionScan {
        data_dir: data_dir::data_root()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        versions_dir: versions_dir
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        versions,
        custom,
    }
}

#[tauri::command]
pub fn game_running(state: State<'_, Mutex<LauncherState>>) -> bool {
    state.lock().unwrap().game_running
}

/// 关闭正在运行的游戏窗口（快速启动页运行中的「取消运行」）。
/// 窗口销毁事件里统一复位运行状态并通知前端。
#[tauri::command]
pub async fn close_game(app: AppHandle, state: State<'_, Mutex<LauncherState>>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(GAME_WINDOW_LABEL) {
        window.close().map_err(|e| format!("关闭游戏窗口失败：{e}"))?;
    } else {
        // 无窗口说明状态失同步，直接复位
        state.lock().unwrap().game_running = false;
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchResult {
    /// 游戏页面地址（本地静态服务器），主要用于调试
    pub url: String,
    pub fullscreen: bool,
}

/// 启动游戏：确保本地静态服务器指向游戏目录，
/// 然后按启动设置以独立窗口打开游戏（全屏或按设置大小的窗口）。
///
/// 必须是 async command：窗口创建需要与主线程事件循环交互，
/// 同步 command 会与其互锁导致应用挂起。
#[tauri::command]
pub async fn launch_game(
    app: AppHandle,
    state: State<'_, Mutex<LauncherState>>,
) -> Result<LaunchResult, String> {
    let settings = settings::load();
    let dir = resolve_game_dir(&settings)
        .ok_or_else(|| "尚未找到游戏目录，请在版本管理中指定版本位置或选择自定义目录".to_string())?;
    if !has_index_html(&dir) {
        return Err(format!("游戏目录中缺少 index.html：{}", dir.display()));
    }

    let fullscreen = settings.launch_mode == LaunchMode::Fullscreen;
    let (window_w, window_h) = windowed_size(&settings);

    let (port, root_changed) = {
        let mut guard = state.lock().unwrap();
        match guard.server.as_ref() {
            Some(server) => {
                let changed = server.root() != dir;
                server.set_root(dir.clone());
                (server.port, changed)
            }
            None => {
                let server = GameServer::start(dir.clone())?;
                let port = server.port;
                guard.server = Some(server);
                (port, true)
            }
        }
    };

    // 用 localhost 而非 127.0.0.1：localhost 在常见代理软件的绕过列表里，
    // 直接写回环 IP 可能被系统代理规则拦截导致加载失败。
    let url = format!("http://localhost:{port}/index.html");
    let parsed_url: tauri::Url = url
        .parse()
        .map_err(|e| format!("游戏地址解析失败：{e}"))?;

    if let Some(existing) = app.get_webview_window(GAME_WINDOW_LABEL) {
        let _ = existing.set_fullscreen(fullscreen);
        if !fullscreen {
            let _ = existing.set_size(tauri::LogicalSize::new(window_w, window_h));
            let _ = existing.center();
        }
        let _ = existing.show();
        let _ = existing.unminimize();
        let _ = existing.set_focus();
        // 服务器根目录已切换（如换版本后再次启动）时，复用窗口需重新加载新目录；
        // 目录未变则不干扰游戏内状态
        if root_changed {
            let _ = existing.navigate(parsed_url.clone());
        }
        state.lock().unwrap().game_running = true;
        return Ok(LaunchResult { url, fullscreen });
    }

    let mut builder = WebviewWindowBuilder::new(&app, GAME_WINDOW_LABEL, WebviewUrl::External(parsed_url))
        .title("光点之旅 · Tour of Light Point")
        .resizable(true)
        .min_inner_size(640.0, 360.0)
        .decorations(true)
        .fullscreen(fullscreen);
    if !fullscreen {
        builder = builder.inner_size(window_w, window_h);
    }
    // 游戏存档（localStorage 等）写入数据文件夹，随启动器目录移动
    if let Ok(saves) = data_dir::saves_dir() {
        builder = builder.data_directory(saves);
    }
    let window = builder
        .center()
        .build()
        .map_err(|e| format!("创建游戏窗口失败：{e}"))?;

    state.lock().unwrap().game_running = true;

    // 游戏窗口销毁时更新运行状态并通知前端恢复「启动游戏」
    let app_handle = app.clone();
    window.on_window_event(move |event| {
        if matches!(event, tauri::WindowEvent::Destroyed) {
            if let Some(state) = app_handle.try_state::<Mutex<LauncherState>>() {
                state.lock().unwrap().game_running = false;
            }
            let _ = app_handle.emit(GAME_CLOSED_EVENT, ());
        }
    });

    Ok(LaunchResult { url, fullscreen })
}

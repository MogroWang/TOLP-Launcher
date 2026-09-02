//! 游戏目录解析与游戏启动（独立游戏窗口）。

use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;

use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::game_server::GameServer;
use crate::settings::{self, LaunchMode, Settings};

pub struct LauncherState {
    pub server: Option<GameServer>,
}

const GAME_WINDOW_LABEL: &str = "game";
const WINDOWED_SIZE: (f64, f64) = (1280.0, 720.0);

/// 解析当前生效的游戏目录：
/// 1. 设置中指定的目录；
/// 2. 可执行文件同目录的 `game` 文件夹（便携版默认约定）；
/// 3. 开发构建下回退到工程根目录的 `game`（便于 `tauri dev` 调试）。
pub fn resolve_game_dir(settings: &Settings) -> Option<PathBuf> {
    if let Some(dir) = settings.game_dir.as_deref() {
        if !dir.trim().is_empty() {
            return Some(PathBuf::from(dir));
        }
    }
    let exe_game = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|parent| parent.join("game")));
    #[cfg(debug_assertions)]
    {
        let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("game");
        if dev.join("index.html").is_file() {
            return Some(dev);
        }
    }
    exe_game
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStatus {
    pub found: bool,
    pub dir: Option<String>,
    pub reason: Option<String>,
}

#[tauri::command]
pub fn game_status() -> GameStatus {
    let settings = settings::load();
    match resolve_game_dir(&settings) {
        Some(dir) if dir.join("index.html").is_file() => GameStatus {
            found: true,
            dir: Some(dir.display().to_string()),
            reason: None,
        },
        Some(dir) if dir.exists() => GameStatus {
            found: false,
            dir: Some(dir.display().to_string()),
            reason: Some("游戏目录中缺少 index.html".to_string()),
        },
        Some(dir) => GameStatus {
            found: false,
            dir: Some(dir.display().to_string()),
            reason: Some("尚未找到游戏文件夹".to_string()),
        },
        None => GameStatus {
            found: false,
            dir: None,
            reason: Some("尚未找到游戏文件夹".to_string()),
        },
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchResult {
    /// 游戏页面地址（本地静态服务器），主要用于调试
    pub url: String,
    pub fullscreen: bool,
}

/// 启动游戏：确保本地静态服务器指向游戏目录，
/// 然后按启动设置以独立窗口打开游戏（全屏或 1280×720 窗口）。
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
        .ok_or_else(|| "尚未找到游戏目录，请先在启动设置中选择".to_string())?;
    if !dir.join("index.html").is_file() {
        return Err(format!("游戏目录中缺少 index.html：{}", dir.display()));
    }

    let fullscreen = settings.launch_mode == LaunchMode::Fullscreen;

    let port = {
        let mut guard = state.lock().unwrap();
        match guard.server.as_ref() {
            Some(server) => {
                server.set_root(dir.clone());
                server.port
            }
            None => {
                let server = GameServer::start(dir.clone())?;
                let port = server.port;
                guard.server = Some(server);
                port
            }
        }
    };

    // 用 localhost 而非 127.0.0.1：localhost 在常见代理软件的绕过列表里，
    // 直接写回环 IP 可能被系统代理规则拦截导致加载失败。
    let url = format!("http://localhost:{port}/index.html");

    if let Some(existing) = app.get_webview_window(GAME_WINDOW_LABEL) {
        let _ = existing.set_fullscreen(fullscreen);
        if !fullscreen {
            let _ = existing.set_size(tauri::LogicalSize::new(WINDOWED_SIZE.0, WINDOWED_SIZE.1));
            let _ = existing.center();
        }
        let _ = existing.show();
        let _ = existing.unminimize();
        let _ = existing.set_focus();
        return Ok(LaunchResult { url, fullscreen });
    }

    let parsed_url: tauri::Url = url
        .parse()
        .map_err(|e| format!("游戏地址解析失败：{e}"))?;

    let mut builder = WebviewWindowBuilder::new(&app, GAME_WINDOW_LABEL, WebviewUrl::External(parsed_url))
        .title("光点之旅 · Tour of Light Point")
        .resizable(true)
        .min_inner_size(640.0, 360.0)
        .decorations(true)
        .fullscreen(fullscreen);
    if !fullscreen {
        builder = builder.inner_size(WINDOWED_SIZE.0, WINDOWED_SIZE.1);
    }
    builder
        .center()
        .build()
        .map_err(|e| format!("创建游戏窗口失败：{e}"))?;

    Ok(LaunchResult { url, fullscreen })
}

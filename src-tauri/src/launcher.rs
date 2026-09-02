//! 游戏目录解析与游戏启动。

use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;

use tauri::State;

use crate::game_server::GameServer;
use crate::settings::{self, LaunchMode, Settings};

pub struct LauncherState {
    pub server: Option<GameServer>,
}

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
    /// 游戏页面地址（本地静态服务器），由前端以内嵌方式加载。
    pub url: String,
    pub fullscreen: bool,
}

/// 启动游戏：确保本地静态服务器指向游戏目录，返回游戏页面地址。
/// 游戏画面由前端以 iframe 内嵌呈现，全屏/窗口化通过主窗口控制。
#[tauri::command]
pub fn launch_game(
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
    Ok(LaunchResult {
        url: format!("http://localhost:{port}/index.html"),
        fullscreen,
    })
}

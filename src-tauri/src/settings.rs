use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaunchMode {
    #[default]
    Fullscreen,
    Windowed,
}

/// 窗口化启动时的游戏窗口大小（像素）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub launch_mode: LaunchMode,
    /// 窗口化启动时的游戏窗口大小；`None` 使用默认 1280×720。
    pub windowed_size: Option<WindowSize>,
    pub game_dir: Option<String>,
    /// 选中的游戏版本：`Some(id)` 为版本启动（数据文件夹版本或内置版本），`None` 为自定义目录启动。
    pub version_id: Option<String>,
    /// 内置版本的自定义位置：数据文件夹中未识别到该版本时使用的游戏目录。
    pub custom_version_dir: Option<String>,
}

/// 便携版约定：设置文件始终放在可执行文件同目录。
pub fn settings_path() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| format!("无法定位可执行文件：{e}"))?;
    let dir = exe
        .parent()
        .ok_or_else(|| "无法定位可执行文件目录".to_string())?;
    Ok(dir.join("launcher-settings.json"))
}

pub fn load() -> Settings {
    settings_path()
        .ok()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

pub fn save(settings: &Settings) -> Result<(), String> {
    let path = settings_path()?;
    let text = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, text).map_err(|e| format!("无法写入设置文件：{e}"))?;
    fs::rename(&tmp, &path).map_err(|e| format!("无法保存设置文件：{e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_settings() -> Settings {
    load()
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<Settings, String> {
    save(&settings)?;
    Ok(settings)
}

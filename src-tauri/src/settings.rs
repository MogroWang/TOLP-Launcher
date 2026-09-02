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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub launch_mode: LaunchMode,
    pub game_dir: Option<String>,
    /// 选中的游戏版本：`Some(id)` 为内置版本（如 4.0.002），`None` 为自定义目录启动。
    pub version_id: Option<String>,
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

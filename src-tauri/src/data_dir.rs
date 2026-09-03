//! 启动器数据文件夹：游戏版本与游戏存档的统一存放位置。
//!
//! 与设置文件一致采用便携版约定，位于可执行文件同目录：
//! `launcher-data/versions/` 存放游戏版本，`launcher-data/saves/` 存放
//! 游戏窗口的浏览器数据（localStorage 存档等），整体随启动器目录移动。

use std::fs;
use std::path::PathBuf;

/// 数据文件夹名（exe 同目录下）。
const DATA_DIR_NAME: &str = "launcher-data";
/// 版本文件夹名：每个版本一个子目录（需包含 index.html）。
pub const VERSIONS_DIR_NAME: &str = "versions";
/// 游戏存档文件夹名：作为游戏窗口的 WebView 数据目录。
pub const SAVES_DIR_NAME: &str = "saves";

fn exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("无法定位可执行文件：{e}"))?
        .parent()
        .map(|parent| parent.to_path_buf())
        .ok_or_else(|| "无法定位可执行文件目录".to_string())
}

/// 数据文件夹根目录：exe 同目录 `launcher-data/`。
pub fn data_root() -> Result<PathBuf, String> {
    Ok(exe_dir()?.join(DATA_DIR_NAME))
}

/// 版本文件夹：`launcher-data/versions/`。
pub fn versions_dir() -> Result<PathBuf, String> {
    Ok(data_root()?.join(VERSIONS_DIR_NAME))
}

/// 游戏存档文件夹：`launcher-data/saves/`。
pub fn saves_dir() -> Result<PathBuf, String> {
    Ok(data_root()?.join(SAVES_DIR_NAME))
}

/// 创建数据文件夹结构（幂等）。失败仅返回错误，不阻塞启动器运行。
pub fn ensure_data_dirs() -> Result<(), String> {
    for dir in [data_root()?, versions_dir()?, saves_dir()?] {
        fs::create_dir_all(&dir).map_err(|e| format!("无法创建数据文件夹 {}：{e}", dir.display()))?;
    }
    Ok(())
}

/// 数据文件夹路径（启动时确保存在，供前端展示）。
#[tauri::command]
pub fn data_dir() -> Result<String, String> {
    ensure_data_dirs()?;
    Ok(data_root()?.display().to_string())
}

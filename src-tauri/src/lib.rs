mod data_dir;
mod game_server;
mod launcher;
mod settings;

use std::sync::Mutex;

use launcher::LauncherState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(LauncherState {
            server: None,
            game_running: false,
        }))
        .setup(|_| {
            // 启动时即创建数据文件夹（游戏版本与存档的存放位置）；失败不阻塞启动
            let _ = data_dir::ensure_data_dirs();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            settings::save_settings,
            launcher::game_status,
            launcher::game_running,
            launcher::launch_game,
            launcher::close_game,
            launcher::list_versions,
            data_dir::data_dir
        ])
        .run(tauri::generate_context!())
        .expect("TOLP 启动器初始化失败");
}

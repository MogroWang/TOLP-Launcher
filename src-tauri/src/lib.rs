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
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            settings::save_settings,
            launcher::game_status,
            launcher::game_running,
            launcher::launch_game
        ])
        .run(tauri::generate_context!())
        .expect("TOLP 启动器初始化失败");
}

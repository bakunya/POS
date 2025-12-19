// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod db;
mod commands;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            
            // Create data dir if not exists
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            }

            tauri::async_runtime::spawn(async move {
                let db = db::setup_db(app_data_dir).await.expect("failed to setup db");
                app_handle.manage(db::AppState { db });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::login,
            commands::start_shift,
            commands::end_shift,
            commands::search_product,
            commands::process_transaction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

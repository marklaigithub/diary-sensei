mod commands;
mod storage;
mod claude;
mod config;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_entries,
            read_entry,
            save_entry,
            save_image,
            delete_entry,
            create_entry_id,
            correct_text,
            translate_text,
            load_config,
            save_config,
            get_entries_dir,
            print_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

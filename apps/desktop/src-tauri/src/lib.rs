pub mod commands;

use crate::commands::mdns;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(mdns::ScanState(Mutex::new(None)))
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![mdns::start_mdns_scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

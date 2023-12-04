// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn is_symlink(path: String) -> bool {
    let metadata = std::fs::symlink_metadata(path).unwrap();
    return metadata.file_type().is_symlink();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, is_symlink])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

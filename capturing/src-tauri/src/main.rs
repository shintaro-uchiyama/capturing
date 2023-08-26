// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let enigo = Enigo::new();
    // let cursor_location: (i32, i32) = Enigo::new()
    // println!(cursor_location);
    return format!(
        "Hello, {}! You've been greeted from Rust!, {:?}",
        name,
        enigo.mouse_location()
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

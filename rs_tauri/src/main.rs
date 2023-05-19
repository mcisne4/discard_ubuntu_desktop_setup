// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use rs_shell;

mod shell_cmds;
use shell_cmds::*;

fn main() {
    rs_shell::hello();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            env_shell,
            is_bash_installed,
            is_zsh_installed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

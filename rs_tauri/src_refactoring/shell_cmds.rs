use rs_shell::env;
use rs_shell::is_installed;

// === ENV === //
#[tauri::command]
pub fn env_shell() -> Result<String, String> {
    env::shell()
}

// === IS INSTALLED === //
#[tauri::command]
pub fn is_installed_bash() -> Result<bool, String> {
    is_installed::bash()
}

#[tauri::command]
pub fn is_installed_zsh() -> Result<bool, String> {
    is_installed::zsh()
}

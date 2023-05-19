use rs_shell::env;
use rs_shell::is_installed;

// === ENV === //
#[tauri::command]
pub fn env_shell() -> Result<String, String> {
    env::get_shell()
}

// === IS INSTALLED === //
#[tauri::command]
pub fn is_bash_installed() -> Result<bool, String> {
    is_installed::bash()
}

#[tauri::command]
pub fn is_zsh_installed() -> Result<bool, String> {
    is_installed::zsh()
}

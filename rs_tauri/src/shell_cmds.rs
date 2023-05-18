use rs_shell::env;

#[tauri::command]
pub fn env_shell() -> Result<String, String> {
    env::get_shell()
}

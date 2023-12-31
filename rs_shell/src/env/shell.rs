use std::env;

pub fn shell() -> Result<String, String> {
    let key = "SHELL";
    let mut shell = env::var(key).map_err(|e| format!("$SHELL: {}", e))?;

    if shell.ends_with("bash") {
        shell = String::from("BASH");
    }

    if shell.ends_with("zsh") {
        shell = String::from("ZSH");
    }

    Ok(shell)
}

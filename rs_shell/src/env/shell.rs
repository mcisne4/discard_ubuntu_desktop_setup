use rs_logger::logging::{log_info, log_map_err};
use std::env;

pub fn shell() -> Result<String, String> {
    let source = "rs_shell|env|shell";
    let err_msg = "Unable to retrieve the environmental 'SHELL' variable";

    let key = "SHELL";
    let mut shell = env::var(key).map_err(|e| log_map_err(&source, err_msg, e, None))?;

    log_info(
        &source,
        &format!("Retrieved env 'SHELL' variable: {}", &shell),
    );

    if shell.ends_with("bash") {
        shell = String::from("BASH");
    }

    if shell.ends_with("zsh") {
        shell = String::from("ZSH");
    }

    Ok(shell)
}

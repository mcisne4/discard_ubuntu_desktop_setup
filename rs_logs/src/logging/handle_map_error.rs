use log::error;
use std::error::Error;

pub fn handle_map_error(
    err: &dyn Error,
    source: &str,
    cause: &str,
    new_err: Option<&str>,
) -> String {
    error!("[{}] {}:\n  {}", source, cause, err.to_string());

    match new_err {
        Some(message) => message.to_owned(),
        None => cause.to_owned(),
    }
}

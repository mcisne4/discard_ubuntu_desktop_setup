use log::{error, info, warn};
use std::error::Error;
use std::path::PathBuf;

pub fn init(log_file_path: PathBuf) -> Result<(), String> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}]{}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(log_file_path).map_err(|e| e.to_string())?)
        .apply()
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn log_info(source: &str, message: &str) {
    info!("[{}] {}", source, message);
}

pub fn log_warn(source: &str, message: &str) {
    warn!("[{}] {}", source, message);
}

pub fn log_error(source: &str, cause: &str, description: &str) {
    error!("[{}] {}:\n  {}", source, cause, description);
}

pub fn log_error_from_map(
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

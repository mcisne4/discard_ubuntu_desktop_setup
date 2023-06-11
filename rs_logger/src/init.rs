mod configure;
mod errors;
mod log_paths;
mod remove_old_logs;

use configure::configure_logger;
pub use errors::InitError;
use log_paths::get_log_paths;
use remove_old_logs::remove_old_logs;
use std::path::PathBuf;

pub fn init(logs_dir: PathBuf) -> Result<(), String> {
    let (logs_dir, log_file, filename) = get_log_paths(logs_dir).map_err(|e| e.to_string())?;

    configure_logger(log_file).map_err(|e| e.to_string())?;

    remove_old_logs(logs_dir, filename);

    Ok(())
}

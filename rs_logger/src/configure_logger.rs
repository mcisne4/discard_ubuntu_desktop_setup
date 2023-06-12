mod errors;
mod initialize_logger;
mod log_paths;
mod remove_old_logs;

use initialize_logger::initialize_logger;
use log_paths::get_log_paths;
use remove_old_logs::remove_old_logs;
use std::path::PathBuf;

pub fn configure_logger(logs_dir: PathBuf) -> Result<(), String> {
    match get_log_paths(logs_dir) {
        Err(e) => Err(e.to_string()),
        Ok((logs_dir, log_file_path, filename)) => {
            initialize_logger(log_file_path).map_err(|e| e.to_string())?;

            remove_old_logs(logs_dir, filename);

            Ok(())
        }
    }
}

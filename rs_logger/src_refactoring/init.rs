use std::path::PathBuf;

mod configure_logger;
mod util;
use configure_logger::configure_logger;
mod remove_old_logs;
use remove_old_logs::remove_old_logs;

pub fn init(log_file_dir: &PathBuf) -> Result<(), String> {
    match configure_logger(log_file_dir) {
        Err(e) => Err(e),
        Ok(_) => {
            remove_old_logs(log_file_dir);

            Ok(())
        }
    }
}

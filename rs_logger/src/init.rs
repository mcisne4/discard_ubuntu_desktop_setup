mod configure;
use std::path::PathBuf;

use configure::configure_logger;

pub fn init(logs_dir: PathBuf) -> Result<(), String> {
    configure_logger(logs_dir).map_err(|e| e.to_string())?;

    Ok(())
}

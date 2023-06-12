mod session_separator;
mod timestamp;

use super::errors::ConfigErrors;
use fern::{log_file, Dispatch};
use session_separator::session_separator;
use std::path::PathBuf;
use timestamp::timestamp;

pub fn initialize_logger(log_file_path: PathBuf) -> Result<(), ConfigErrors> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}",
                timestamp(),
                record.level(),
                message
            ))
        })
        .chain(log_file(&log_file_path).map_err(|e| ConfigErrors::ConfigLogFile(e))?)
        .apply()
        .map_err(|e| ConfigErrors::ConfigDispatch(e))?;

    session_separator(&log_file_path);

    Ok(())
}

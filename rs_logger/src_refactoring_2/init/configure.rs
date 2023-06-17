use fern::{log_file, Dispatch};
use std::path::PathBuf;

mod timestamp;

use super::InitError;
use crate::InfoLog;
use timestamp::timestamp;

pub fn configure_logger(log_file_path: PathBuf) -> Result<(), InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}",
                timestamp(),
                record.level(),
                message
            ))
        })
        .chain(log_file(&log_file_path).map_err(|e| InitError::ConfigLogFile(e))?)
        .apply()
        .map_err(|e| InitError::ConfigDispatch(e))?;

    InfoLog::Id101101(log_file_path.to_owned()).log();

    Ok(())
}

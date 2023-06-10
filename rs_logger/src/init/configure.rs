use fern::{log_file, Dispatch};
use std::path::PathBuf;

mod errors;
mod filename;
mod timestamp;
mod validate_dir;

pub use errors::InitError;
use filename::generate_filename;
use timestamp::timestamp;
use validate_dir::validate_logs_dir;

pub fn configure_logger(logs_dir: PathBuf) -> Result<(), InitError> {
    let mut log_file_path = validate_logs_dir(logs_dir)?;
    let filename = generate_filename()?;
    log_file_path.push(filename);

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}",
                timestamp(),
                record.level(),
                message
            ))
        })
        .chain(log_file(log_file_path).map_err(|e| InitError::ConfigLogFile(e))?)
        .apply()
        .map_err(|e| InitError::ConfigDispatch(e))?;

    Ok(())
}

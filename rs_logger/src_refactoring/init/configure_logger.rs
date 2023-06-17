use super::util::{extract_dir, generate_filename, timestamp};
use crate::log::LogInfo;
use std::path::PathBuf;

pub fn configure_logger(log_file_dir: &PathBuf) -> Result<(), String> {
    let mut log_file_path = extract_dir(log_file_dir)?;

    let log_filename = generate_filename()?;
    log_file_path.push(log_filename);

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}]{}",
                timestamp(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(&log_file_path).map_err(|e| e.to_string())?)
        .apply()
        .map_err(|e| e.to_string())?;

    LogInfo::Code1001(log_file_path).log();

    Ok(())
}

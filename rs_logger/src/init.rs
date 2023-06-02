use crate::logging::log_info;
use crate::util::{extract_dir, get_timestamp, log_empty_line, log_filename};
use std::path::PathBuf;

pub fn init(log_file_dir: &PathBuf) -> Result<(), String> {
    let mut log_file_path = extract_dir(log_file_dir)?;

    let log_filename = log_filename()?;
    log_file_path.push(log_filename);

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}]{}",
                get_timestamp(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(&log_file_path).map_err(|e| e.to_string())?)
        .apply()
        .map_err(|e| e.to_string())?;

    let source = "rs_logs|init";
    log_empty_line(&log_file_path);
    log_info(&source, "===== NEW SESSION =====");
    log_info(&source, "Logging initialized");

    Ok(())
}

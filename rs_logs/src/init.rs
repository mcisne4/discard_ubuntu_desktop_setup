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

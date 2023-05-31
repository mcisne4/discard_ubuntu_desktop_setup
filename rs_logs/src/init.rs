use crate::util::{get_timestamp, log_filename};
use std::path::PathBuf;

pub fn init(log_file_dir: PathBuf) -> Result<(), String> {
    let filename = log_filename()?;
    let mut log_file_path = log_file_dir;
    match log_file_path.is_dir() {
        true => log_file_path.push(filename),
        false => log_file_path.set_file_name(filename),
    }
    println!("FILEPATH:\n  '{}'", log_file_path.display());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}]{}",
                get_timestamp(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(log_file_path).map_err(|e| e.to_string())?)
        .apply()
        .map_err(|e| e.to_string())?;

    Ok(())
}

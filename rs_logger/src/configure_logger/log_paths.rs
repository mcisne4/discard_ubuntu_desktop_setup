mod generate_filename;
mod validate_logs_dir;

use super::errors::ConfigErrors;
use generate_filename::generate_filename;
use std::path::PathBuf;
use validate_logs_dir::validate_logs_dir;

pub fn get_log_paths(logs_dir: PathBuf) -> Result<(PathBuf, PathBuf, String), ConfigErrors> {
    let logs_dir = validate_logs_dir(logs_dir)?;

    let mut log_file = logs_dir.clone();
    let filename = generate_filename()?;
    log_file.push(&filename);

    Ok((logs_dir, log_file, filename))
}

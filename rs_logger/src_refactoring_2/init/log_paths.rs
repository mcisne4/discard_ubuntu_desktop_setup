mod filename;
use std::path::PathBuf;

use filename::generate_filename;

mod validate_dir;
use validate_dir::validate_logs_dir;

use super::InitError;

pub fn get_log_paths(logs_dir: PathBuf) -> Result<(PathBuf, PathBuf, String), InitError> {
    let logs_dir = validate_logs_dir(logs_dir)?;

    let mut log_file = logs_dir.clone();
    let filename = generate_filename()?;
    log_file.push(&filename);

    Ok((logs_dir, log_file, filename))
}

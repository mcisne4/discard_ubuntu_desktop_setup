use super::errors::InitError;
use std::path::PathBuf;

pub fn validate_logs_dir(logs_dir: PathBuf) -> Result<PathBuf, InitError> {
    let original_path = logs_dir.clone();

    let mut logs_dir = logs_dir;

    if logs_dir.is_file() {
        logs_dir.pop();
    }

    if !logs_dir.is_dir() {
        return Err(InitError::LogDirectory(original_path));
    }

    Ok(logs_dir)
}

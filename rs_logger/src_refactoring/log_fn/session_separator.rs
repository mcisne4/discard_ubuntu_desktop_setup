use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn session_separator(log_file_path: &PathBuf) {
    if !log_file_path.is_file() {
        return;
    }

    match log_file_path.metadata() {
        Ok(metadata) => {
            if metadata.len() == 0 {
                return;
            }
        }
        Err(_) => return,
    };

    match OpenOptions::new().append(true).open(log_file_path) {
        Err(_) => return,
        Ok(mut file) => {
            file.write_all("\n".as_bytes()).unwrap_or(());
        }
    }
}

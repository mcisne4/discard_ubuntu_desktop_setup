use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn log_empty_line(log_file_path: &PathBuf) {
    if !log_file_path.is_file() {
        return;
    }

    match OpenOptions::new().append(true).open(log_file_path) {
        Err(_) => return,
        Ok(mut file) => {
            file.write_all("\n".as_bytes()).unwrap_or(());
        }
    }
}

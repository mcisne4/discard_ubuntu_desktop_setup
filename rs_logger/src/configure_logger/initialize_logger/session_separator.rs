use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub fn session_separator(log_file_path: &PathBuf) {
    if !log_file_path.is_file() {
        return;
    }

    match log_file_path.metadata() {
        Err(_) => return,
        Ok(metadata) => {
            if metadata.len() == 0 {
                return;
            }
        }
    }

    match OpenOptions::new().append(true).open(log_file_path) {
        Err(_) => return,
        Ok(mut file) => file.write_all("\n".as_bytes()).unwrap_or(()),
    }
}

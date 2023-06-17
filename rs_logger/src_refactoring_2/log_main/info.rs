use super::util::session_separator;
use crate::log_functions::info_log;
use std::path::PathBuf;

pub enum InfoLog {
    Id000101,
    Id101101(PathBuf),
    Id102101,
    Id102102(PathBuf),
    Id102103(usize),
}
impl InfoLog {
    pub fn log(self) {
        match self {
            Self::Id000101 => info_log("000", "Test Info Log"),
            Self::Id101101(log_file_path) => {
                session_separator(&log_file_path);
                info_log("101101", "===== New Session =====");
                info_log(
                    "101101",
                    format!(
                        "Logger Initialized:\n\tLog File: '{}'",
                        log_file_path.display()
                    ),
                );
            }
            Self::Id102101 => info_log("102101", "Looking for old log files"),
            Self::Id102102(extra_file) => info_log(
                "102102",
                format!(
                    "Log file successfully dropped:\n\tLog File: '{}'",
                    extra_file.display()
                ),
            ),
            Self::Id102103(file_count) => info_log(
                "102103",
                format!(
                    "Found {} log files. No files need to be dropped",
                    file_count
                ),
            ),
        }
    }
}

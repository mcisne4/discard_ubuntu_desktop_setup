use super::util::session_separator;
use crate::log_functions::info_log;
use std::path::PathBuf;

pub enum InfoLog {
    Id000101,
    Id101101(PathBuf),
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
        }
    }
}

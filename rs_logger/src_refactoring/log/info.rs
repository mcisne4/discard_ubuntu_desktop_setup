use crate::log_fn::{log_info, session_separator};
use std::path::PathBuf;

pub enum LogInfoSource {
    Code1001,
    Code1010,
    Code1011,
}
impl LogInfoSource {
    pub fn source(self) -> String {
        let source = match self {
            Self::Code1001 => "rs_logger::init::configure_logger::configure_logger()",
            Self::Code1010 => "rs_logger::init::remove_old_logs::remove_old_logs()",
            Self::Code1011 => "rs_logger::init::remove_old_logs::remove_old_logs()",
        };

        source.to_owned()
    }
}

pub enum LogInfo {
    Code1001(PathBuf),
    Code1010,
    Code1011(usize),
    Code1012(PathBuf),
}
impl LogInfo {
    pub fn log(self) {
        match self {
            Self::Code1001(log_file_path) => {
                session_separator(&log_file_path);
                log_info(1001, "===== New Session =====");
                log_info(
                    1001,
                    format!(
                        "Logger Initialized:\n\tLog File: '{}'",
                        log_file_path.display()
                    ),
                );
            }
            Self::Code1010 => log_info(1010, "Looking for old log files"),
            Self::Code1011(file_count) => log_info(
                1011,
                format!(
                    "Found {} log files. No files need to be dropped",
                    file_count
                ),
            ),
            Self::Code1012(file) => log_info(
                1012,
                format!(
                    "Log file successfully dropped:\n\tLog File: '{}'",
                    file.display()
                ),
            ),
        }
    }
}

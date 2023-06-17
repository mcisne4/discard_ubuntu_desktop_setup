use crate::log_fn::log_warning;
use std::path::PathBuf;

pub enum LogWarningSource {
    Code1010,
    Code1011,
}
impl LogWarningSource {
    pub fn source(self) -> String {
        let source = match self {
            Self::Code1010 => "rs_logger::init::remove_old_logs::remove_old_logs()",
            Self::Code1011 => "rs_logger::init::remove_old_logs::remove_old_logs()",
        };

        source.to_owned()
    }
}

pub enum LogWarning {
    Code1010(PathBuf),
    Code1011(PathBuf, String),
    Code1012(usize, usize),
    Code1013(PathBuf),
}
impl LogWarning {
    pub fn log(self) {
        match self {
            Self::Code1010(dir) => log_warning(
                1001,
                format!(
                    "ABORTING: The provided path is not a directory:\n\tPath: '{}'",
                    dir.display()
                ),
            ),
            Self::Code1011(dir, details) => log_warning(
                1010,
                format!(
              "ABORTING: Unable to read the directory contents:\n\tPath: '{}'\n\tDetails: '{}'",
              dir.display(),
              details,
            ),
            ),
            Self::Code1012(file_count, max_files) => log_warning(
                1012,
                format!(
                    "Found {} log files. Keeping the newest {} files and dropping the rest",
                    file_count, max_files
                ),
            ),
            Self::Code1013(file) => log_warning(
                1013,
                format!(
                    "Log file successfully dropped:\n\tLog File: '{}'",
                    file.display()
                ),
            ),
        }
    }
}

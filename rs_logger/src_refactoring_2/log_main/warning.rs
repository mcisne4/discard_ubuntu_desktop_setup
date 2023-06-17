use std::path::PathBuf;

use crate::log_functions::warning_log;

pub enum WarningLog {
    Id000201,
    Id102201(String),
    Id102202(usize, usize),
    Id102203(PathBuf, String),
}
impl WarningLog {
    pub fn log(&self) {
        match self {
            Self::Id000201 => warning_log("000201", "Test Warning Log"),
            Self::Id102201(details) => {
                let cause = "Unable to read the contents of the logs directory";
                warning_log("102201", format!("{}:\n\t{}", &cause, details));
            }
            Self::Id102202(file_count, max_files) => warning_log(
                "102202",
                format!(
                    "Fount {} log files. Keeping the newest {} files and dropping the rest",
                    file_count, max_files
                ),
            ),
            Self::Id102203(log_file, details) => warning_log(
                "102203",
                format!(
                    "WARNING: Could not remove log file:\n\tLog File: '{}'\n\t{}",
                    log_file.display(),
                    details
                ),
            ),
        }
    }
}

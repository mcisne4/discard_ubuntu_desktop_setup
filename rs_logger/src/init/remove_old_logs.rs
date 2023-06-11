use lazy_regex::regex;
use std::fs::remove_file;
use std::path::PathBuf;

use crate::{InfoLog, WarningLog};

const MAX_LOG_FILES: usize = 6;

pub fn remove_old_logs(logs_dir: PathBuf, log_filename: String) {
    InfoLog::Id102101.log();

    match logs_dir.read_dir() {
        Err(e) => {
            WarningLog::Id102201(e.to_string());
            return;
        }
        Ok(dir_contents) => {
            let filename_regex = regex!("(\\d+)_info.log");

            let mut log_files = dir_contents
                .into_iter()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|entry| {
                    if let Some(filename) = entry.file_name() {
                        if let Some(filename) = filename.to_str() {
                            if filename == log_filename.as_str() {
                                return false;
                            }

                            return filename_regex.is_match(filename);
                        }
                    }

                    false
                })
                .collect::<Vec<_>>();
            log_files.sort();
            log_files.reverse();

            if log_files.len() + 1 > MAX_LOG_FILES {
                WarningLog::Id102202(log_files.len() + 1, MAX_LOG_FILES).log();

                let start = MAX_LOG_FILES - 1;
                let extra_log_files = log_files.drain(start..).collect::<Vec<_>>();

                for file in extra_log_files {
                    match remove_file(file.clone()) {
                        Ok(_) => InfoLog::Id102102(file).log(),
                        Err(e) => WarningLog::Id102203(file, e.to_string()).log(),
                    }
                }
            } else {
                InfoLog::Id102103(log_files.len() + 1).log();
            }
        }
    }
}

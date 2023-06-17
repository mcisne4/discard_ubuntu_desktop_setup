// use crate::logging::{info, warn};
// use crate::util::extract_dir;
use lazy_regex::regex;
use std::fs::{remove_file, ReadDir};
use std::path::PathBuf;

use super::util::extract_dir;
use crate::log::{LogInfo, LogWarning};

const MAX_LOG_FILES: usize = 6;

pub fn remove_old_logs(log_file_dir: &PathBuf) {
    LogInfo::Code1010.log();

    let mut log_file_dir = log_file_dir;
    match extract_dir(&log_file_dir) {
        Ok(path) => log_file_dir = &path,
        Err(_) => {
            LogWarning::Code1010(log_file_dir.to_owned()).log();
            return;
        }
    };

    match log_file_dir.read_dir() {
        Err(e) => {
            LogWarning::Code1011(log_file_dir.to_owned(), e.to_string()).log();
            return;
        }
        Ok(dir_contents) => prune_files(dir_contents),
    }
}

fn prune_files(dir_contents: ReadDir) {
    let filename_regex = regex!("(\\d+)_info.log");

    let mut log_files = dir_contents
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| {
            if let Some(filename) = entry.file_name() {
                if let Some(filename) = filename.to_str() {
                    return filename_regex.is_match(filename);
                }
            }

            false
        })
        .collect::<Vec<_>>();
    log_files.sort();
    log_files.reverse();

    if log_files.len() > MAX_LOG_FILES {
        LogWarning::Code1012(log_files.len(), MAX_LOG_FILES).log();

        let extra_log_files = log_files.drain(MAX_LOG_FILES..).collect::<Vec<_>>();

        for file in extra_log_files {
            match remove_file(file.clone()) {
                Ok(_) => {
                    LogInfo::Code1012(file).log();
                }
                Err(e) => {
                    let message = format!(
                        "WARNING: Could not remove file:\n  File: '{}'\n  {}",
                        file.display(),
                        e.to_string()
                    );
                    warn(&source, &message);
                }
            }
        }
    } else {
        let message = format!(
            "Found {} log files. No files need to be dropped",
            log_files.len()
        );
        info(&source, &message);
    }
}

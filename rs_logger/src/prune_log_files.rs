use crate::logging::{log_info, log_warn};
use crate::util::extract_dir;
use lazy_regex::regex;
use std::fs::{remove_file, ReadDir};
use std::path::PathBuf;

const MAX_LOG_FILES: usize = 6;

pub fn prune_log_files(log_file_dir: PathBuf) {
    let source = "rs_logs|prune_log_files";

    let mut log_file_dir = log_file_dir;
    match extract_dir(&log_file_dir) {
        Ok(path) => log_file_dir = path,
        Err(_) => {
            let message = format!(
                "ABORTING: The provided path is not a directory:\n  Path: '{}'",
                log_file_dir.display()
            );
            log_warn(&source, &message);
            return;
        }
    };

    log_info(&source, "Removing old log files");

    match log_file_dir.read_dir() {
        Err(e) => {
            let message = format!(
                "ABORTING: Unable to read the provided directory:\n  Path: '{}'\n  {}",
                log_file_dir.display(),
                e.to_string()
            );
            log_warn(source, &message);
            return;
        }
        Ok(dir_contents) => prune_files(dir_contents, source),
    }
}

fn prune_files(dir_contents: ReadDir, source: &str) {
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
        let message = format!(
            "Found {} log files. Keeping the newest {} files and dropping the rest",
            log_files.len(),
            MAX_LOG_FILES
        );
        log_info(&source, &message);

        let extra_log_files = log_files.drain(MAX_LOG_FILES..).collect::<Vec<_>>();

        for file in extra_log_files {
            match remove_file(file.clone()) {
                Ok(_) => {
                    let message = format!(
                        "Log file removed successfully:\n  File: '{}'",
                        file.display()
                    );
                    log_info(&source, &message);
                }
                Err(e) => {
                    let message = format!(
                        "WARNING: Could not remove file:\n  File: '{}'\n  {}",
                        file.display(),
                        e.to_string()
                    );
                    log_warn(&source, &message);
                }
            }
        }
    } else {
        let message = format!(
            "Found {} log files. No files need to be dropped",
            log_files.len()
        );
        log_info(&source, &message);
    }
}

use crate::Logs;
use lazy_regex::regex;
use std::{fs::remove_file, path::PathBuf};

const MAX_LOG_FILES: usize = 6;

pub fn remove_old_logs(logs_dir: PathBuf, filename: String) {
    let mut logger = Logs::RsLogger01.as_logger();

    logger.log_info("Looking for old log files");

    match logs_dir.read_dir() {
        Err(e) => {
            let msg = format!(
                "Unable to read the contents of the logs directory\n\t{}",
                e.to_string()
            );
            logger.log_warn(msg);
            return;
        }
        Ok(dir_contents) => {
            let filename_regex = regex!("(\\d+)_info.log");

            let mut log_files = dir_contents
                .into_iter()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|entry| {
                    if let Some(filename_os) = entry.file_name() {
                        if let Some(current_filename) = filename_os.to_str() {
                            if current_filename == filename.as_str() {
                                return false;
                            }

                            return filename_regex.is_match(current_filename);
                        }
                    }

                    false
                })
                .collect::<Vec<_>>();
            log_files.sort();
            log_files.reverse();

            if log_files.len() + 1 > MAX_LOG_FILES {
                let msg = format!(
                    "Found {} log files. Keeping the newest {} files and dropping the rest",
                    log_files.len() + 1,
                    MAX_LOG_FILES
                );
                logger.log_warn(msg);

                let start = MAX_LOG_FILES - 1;
                let extra_log_files = log_files.drain(start..).collect::<Vec<_>>();

                for file in extra_log_files {
                    match remove_file(file.clone()) {
                        Ok(_) => {
                            let msg = format!(
                                "Log file successfully dropped:\n\tLog File: '{}'",
                                file.display()
                            );
                            logger.log_info(msg);
                        }
                        Err(e) => logger.log_warn(format!(
                            "WARNING: Could not remove log file:\n\tLog File: '{}'\n\tDetails: {}",
                            file.display(),
                            e.to_string()
                        )),
                    }
                }
            } else {
                logger.log_info(format!(
                    "Found {} log files. No files need to be dropped",
                    log_files.len()
                ));
            }
        }
    }
}

// enum FileLogs {
//     Log01,
//     Log02,
// }
// impl FileLogs {
//     fn log(&self) {
//         let msg = match self {
//             Self::Log01 => (1_u8, "Hello"),
//             Self::Log02 => (2_u8, "Another"),
//         };
//     }
// }

use rs_logger::{init, prune_log_files};
use std::env;

pub fn initialize() {
    // --- Dev Logs Path --- //
    let mut log_files_dir = env::current_dir().unwrap();
    log_files_dir.push("dev_logs");

    // --- Init --- //
    match init(&log_files_dir) {
        Ok(_) => println!("Logger Initialization: Successful\n"),
        Err(e) => println!("ERROR: Logger Initialization:\n  {}", e),
    }

    prune_log_files(log_files_dir);
}

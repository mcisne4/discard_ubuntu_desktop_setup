use rs_logger::{init, prune_log_files};
use std::fs::write;
use std::{env, path::PathBuf};

pub fn startup_functions() {
    // --- Dev Logs Path --- //
    let mut log_files_dir = env::current_dir().unwrap();
    log_files_dir.push("dev_logs");
    println!("Dev Logs Directory:\n  '{}'\n", &log_files_dir.display());

    // --- Init --- //
    match init(&log_files_dir) {
        Ok(_) => println!("Logger Initialization: Successful\n"),
        Err(e) => println!("ERROR: Logger Initialization:\n  {}", e),
    }

    // --- Prune Old Log Files --- //
    create_extra_log_files(&log_files_dir);

    println!("Removing old log files\n");
    prune_log_files(log_files_dir);
}

fn create_extra_log_files(log_file_dir: &PathBuf) {
    println!("Creating additional log files");

    if !log_file_dir.is_dir() {
        println!(
            "ERROR: The provided path is not a directory:\n  {}",
            log_file_dir.display()
        );
        return;
    }

    let filenames: Vec<&str> = vec![
        "20230101_info.log",
        "20221104_info.log",
        "20221015_info.log",
        "20210824_info.log",
        "20202020_info.log",
        "20180229_info.log",
        "20151230_info.log",
        "19980614_info.log",
    ];

    for filename in filenames {
        let mut file_path = log_file_dir.clone();
        file_path.push(filename);

        match write(file_path, "") {
            Ok(_) => println!("  File '{}' created", filename),
            Err(e) => println!(
                "  ERROR: Could not write to '{}':\n  {}",
                filename,
                e.to_string()
            ),
        }
    }
}

// use rs_logs::logging::{log_error, log_info, log_map_err, log_warn};
// use rs_logs::{init, prune_log_files};
// use std::fs::write;
// use std::{env, path::PathBuf};

// use super::errors::LogError;

mod startup;
use startup::startup_functions;

mod logs_standard;
use logs_standard::starndard_logs;

mod logs_err_conversion;
use logs_err_conversion::log_error_conversions;

mod initialize;
use initialize::initialize;

pub enum LoggerMode {
    FullDev,
    Init,
    DontRun,
}

pub fn dev_rs_logger(mode: LoggerMode) {
    match mode {
        LoggerMode::DontRun => return,
        LoggerMode::FullDev => {
            println!("--------------------------------------");
            println!("--- Developing for Crate: 'rs_log' ---");
            println!("--------------------------------------\n");

            // --- Startup Initialization --- //
            startup_functions();

            // --- Standard Logs --- //
            starndard_logs();

            // --- Logging and Converting Errors --- //
            log_error_conversions();
        }
        LoggerMode::Init => initialize(),
    }
}

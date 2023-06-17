mod startup;
use startup::startup_functions;

mod logs_standard;
use logs_standard::starndard_logs;

mod logs_err_conversion;
use logs_err_conversion::log_error_conversions;

pub enum LoggerMode {
    FullDev,
    InitLogger,
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
            startup_functions(true);

            // --- Standard Logs --- //
            starndard_logs();

            // --- Logging and Converting Errors --- //
            log_error_conversions();
        }
        LoggerMode::InitLogger => startup_functions(false),
    }
}

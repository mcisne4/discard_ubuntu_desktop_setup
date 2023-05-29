use rs_logs::{handle_map_error, init, log_error, log_info, log_warn};
use std::env;

use super::errors::LogError;

pub fn dev(run: bool) {
    if !run {
        return;
    }

    println!("--------------------------------------");
    println!("--- Developing for Crate: 'rs_log' ---");
    println!("--------------------------------------\n");

    // --- Dev Logs Path --- //
    let mut log_file = env::current_dir().unwrap();
    log_file.push("dev_logs");
    log_file.push("info.log");
    println!("Dev Logs Directory:\n  '{}'\n", &log_file.display());

    // --- Init --- //
    match init(log_file) {
        Ok(_) => println!("Logger Initialization: Successful\n"),
        Err(e) => println!("ERROR: Logger Initialization:\n  {}", e),
    }

    // --- Writting Tests --- //

    // ___ Log: INFO ___ //
    let msg = "An info statement";
    println!("Add 'INFO' log:\n  '{}'\n", msg);
    log_info("rs_dev::rs_logs", msg);

    // ___ Log: WARNING ___ //
    let msg = "A warning statement";
    println!("Add 'warning' log:\n  '{}'\n", msg);
    log_warn("rs_dev::rs_logs", msg);

    // ___ Log: ERROR ___ //
    let cause = "The cause of the error";
    let description = "The description for the error statement";
    println!("Add 'error' log:\n  {}:\n    {}\n", cause, description);
    log_error("rs_dev::rs_logs", cause, description);

    // ___ Log: ERROR Handler ___ //
    fn failing_fn() -> Result<(), LogError> {
        let fail = true;
        match fail {
            true => Err(LogError::AnError(String::from("forced fail function"))),
            false => Ok(()),
        }
    }

    fn failing_fn_handler() -> Result<(), String> {
        failing_fn().map_err(|e| {
            handle_map_error(
                &e,
                "rs_dev::rs_log",
                "Catching an error with 'map_err()'",
                Some("This is an intercepted error message"),
            )
        })?;
        Ok(())
    }

    println!("Add 'error' log from error handling:");
    match failing_fn_handler() {
        Ok(_) => (),
        Err(e) => println!("  Handled Error:\n    {}\n", e),
    }
}

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

pub fn dev(run: bool) {
    if !run {
        return;
    }

    println!("--------------------------------------");
    println!("--- Developing for Crate: 'rs_log' ---");
    println!("--------------------------------------\n");

    // --- Startup Initialization --- //
    startup_functions();

    // --- Standard Logs --- //
    starndard_logs();

    // --- Logging and Converting Errors --- //
    log_error_conversions();

    // // --- Writting Tests --- //
    // let source = "rs_dev|rs_logs";

    // // ___ Log: INFO ___ //
    // let msg = "An info statement";
    // println!("Push an 'INFO' log: '{}'\n", msg);
    // log_info(&source, msg);

    // // ___ Log: WARNING ___ //
    // let msg = "A warning statement";
    // println!("Push a 'WARN' log: '{}'\n", msg);
    // log_warn(&source, msg);

    // // ___ Log: ERROR ___ //
    // let cause = "The cause of the error";
    // let description = "A warning statement";
    // println!(
    //     "Push an 'ERROR' log:\n  cause: '{}'\n  description: '{}'\n",
    //     cause, description
    // );
    // log_error(&source, cause, description);

    // // ___ Log: ERROR map_err ___ //
    // let cause = "Error converted with 'log_map_err()'";
    // let response = "Intercepted error response";
    // match failing_fn().map_err(|e| {
    //     println!(
    //         "Push an 'ERROR' log from 'map_err' conversion:\n  cause: '{}'\n  description: '{}'",
    //         cause,
    //         e.to_string()
    //     );
    //     log_map_err(&source, cause, e, Some(response))
    // }) {
    //     Ok(_) => (),
    //     Err(e) => println!("  result: '{}'\n", e),
    // }
}

// fn failing_fn() -> Result<(), LogError> {
//     let fail = true;
//     match fail {
//         true => Err(LogError::AnError(String::from("forced fail function"))),
//         false => Ok(()),
//     }
// }

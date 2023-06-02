use crate::util::errors::LogError;
use rs_logger::logging::log_map_err;

pub fn log_error_conversions() {
    let source = "rs_dev|rs_logs";

    // --- Log: ERROR conversion | Error Description --- //
    let cause = "Error converted in 'map_err()' using the ERROR as the description";
    match failing_fn().map_err(|e| {
        println!(
            "Push an 'ERROR' log from 'map_err' conversion:\n  cause: '{}'\n  description: '{}'",
            cause,
            e.to_string()
        );
        log_map_err(&source, cause, e, None)
    }) {
        Ok(_) => (),
        Err(e) => println!("  result: '{}'\n", e),
    }

    // --- Log: ERROR conversion | Custom Description --- //
    let cause = "Error converted in 'map_error()' using a Custom description";
    let description = "Using a custom description";
    match failing_fn().map_err(|_e| {
        println!(
            "Push an 'ERROR' log from 'map_err' conversion:\n  cause: '{}'\n  description: '{}'",
            cause, description
        );
        log_map_err(&source, cause, description, None)
    }) {
        Ok(_) => (),
        Err(e) => println!("  result: '{}'\n", e),
    }

    // --- Log: Error conversion | Replace Error Result --- //
    let cause = "Error converted in 'map_error()' and  replacing the Result Error";
    let response = "The returned error has been replaced";
    match failing_fn().map_err(|e| {
        println!(
            "Push an 'ERROR' log from 'map_err' conversion:\n  cause: '{}'\n  description: '{}'",
            cause,
            e.to_string()
        );
        log_map_err(&source, cause, e, Some(response))
    }) {
        Ok(_) => (),
        Err(e) => println!("  result: '{}'\n", e),
    }
}

fn failing_fn() -> Result<(), LogError> {
    let fail = true;
    match fail {
        true => Err(LogError::AnError(String::from("failing function"))),
        false => Ok(()),
    }
}

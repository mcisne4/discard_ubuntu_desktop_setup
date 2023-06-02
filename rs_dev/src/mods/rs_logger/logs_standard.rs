use rs_logger::logging::{log_error, log_info, log_warn};

pub fn starndard_logs() {
    let source = "rs_dev|rs_logs";

    // --- Log: INFO --- //
    let msg = "An info statement";
    println!("Push an 'INFO' log: '{}'\n", msg);
    log_info(&source, msg);

    // --- Log: WARNING --- //
    let msg = "A warning statement";
    println!("Push a 'WARN' log: '{}'\n", msg);
    log_warn(&source, msg);

    // --- Log: ERROR --- //
    let cause = "The cause of the error";
    let description = "An error statement";
    println!(
        "Push an 'ERROR' log:\n  cause: '{}'\n  description: '{}'\n",
        cause, description
    );
    log_error(&source, cause, description);
}

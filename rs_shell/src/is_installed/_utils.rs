use rs_logger::logging::{log_error, log_info};
use run_script;

pub fn command_v_cmd(cmd: &str) -> Result<(i32, String, String), String> {
    let script = format!(
        "command -v {} 2>&1 || {{ echo \"The command '{}' is not available\"; exit 1; }}",
        &cmd, &cmd
    );

    let source = format!("rs_shell|is_installed|{}", cmd);

    match run_script::run_script!(script) {
        Ok((code, stdout, stderr)) => {
            log_info(
                &source,
                &format!(
                    "Checking if '{}' is installed:\n  {{code: {}, stdout: {}, stderr: {}}}",
                    cmd, &code, &stdout, &stderr
                ),
            );
            Ok((code, stdout, stderr))
        }
        Err(e) => {
            let cause = format!("Unable to check if '{}' is installed", cmd);
            log_error(&source, &cause, &e.to_string());
            Err(cause)
        }
    }
}

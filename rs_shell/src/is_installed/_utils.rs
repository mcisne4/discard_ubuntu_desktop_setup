use run_script;

pub fn command_v_cmd(cmd: &str) -> Result<(i32, String, String), String> {
    let script = format!(
        "command -v {} 2>&1 || {{ echo \"The command '{}' is not available\"; exit 1; }}",
        cmd, cmd
    );

    match run_script::run_script!(script) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

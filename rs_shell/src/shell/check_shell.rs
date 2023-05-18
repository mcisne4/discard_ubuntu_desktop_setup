use crate::types::ShellResponse;
use run_script;

pub fn check_shell() -> Result<ShellResponse, String> {
    let script = r#"
    echo $SHELL
    "#;
    let options = run_script::ScriptOptions::new();
    let args: Vec<String> = vec![];

    let cmd_results = run_script::run(script, &args, &options);

    match cmd_results {
        Ok((code, mut stdout, stderr)) => {
            if code == 0 {
                let output = stdout.trim_end().to_string();

                if output.ends_with("bash") {
                    stdout = String::from("BASH");
                }

                if output.ends_with("zsh") {
                    stdout = String::from("ZSH");
                }
            }

            Ok(ShellResponse {
                code,
                stdout,
                stderr,
            })
        }
        Err(e) => Err(e.to_string()),
    }
}

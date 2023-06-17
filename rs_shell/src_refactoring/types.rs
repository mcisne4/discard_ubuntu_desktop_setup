#[derive(Debug)]
pub struct ShellResponse {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

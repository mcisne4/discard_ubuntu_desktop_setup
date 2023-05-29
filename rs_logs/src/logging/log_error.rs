use log::error;

pub fn log_error(source: &str, cause: &str, description: &str) {
    error!("[{}] {}:\n  {}", source, cause, description);
}

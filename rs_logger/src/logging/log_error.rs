use log::error;

pub fn log_error(source: &str, cause: &str, description: &str) {
    error!("[{}] {}:\n\tdescription: {}", source, cause, description);
}

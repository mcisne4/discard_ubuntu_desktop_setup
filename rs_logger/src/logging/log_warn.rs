use log::warn;

pub fn log_warn(source: &str, message: &str) {
    warn!("[{}] {}", source, message);
}

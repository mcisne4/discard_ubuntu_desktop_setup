use log::info;

pub fn log_info(source: &str, message: &str) {
    info!("[{}] {}", source, message);
}

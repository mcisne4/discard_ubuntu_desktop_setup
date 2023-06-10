use log::error;

pub fn error_log<S>(log_id: &str, data: S)
where
    S: AsRef<str> + std::fmt::Display,
{
    error!("{}] {}", log_id, data);
}

use log::warn;

pub fn warning_log<S>(log_id: &str, data: S)
where
    S: AsRef<str> + std::fmt::Display,
{
    warn!("{}] {}", log_id, data);
}

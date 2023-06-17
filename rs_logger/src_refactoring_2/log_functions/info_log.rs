use log::info;

pub fn info_log<S>(log_id: &str, data: S)
where
    S: AsRef<str> + std::fmt::Display,
{
    info!("{}] {}", log_id, data);
}

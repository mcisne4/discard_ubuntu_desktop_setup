use log::info;

pub fn log_error<S>(id: u16, data: S)
where
    S: AsRef<str> + std::fmt::Display,
{
    info!("[{}] {}", id, data);
}

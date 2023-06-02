use core::str;
use log::error;

pub fn log_map_err<T>(source: &str, cause: &str, err_or_desc: T, new_err: Option<&str>) -> String
where
    T: std::fmt::Display,
{
    error!(
        "[{}] {}:\n\tdescription: {}",
        source,
        cause,
        err_or_desc.to_string()
    );

    match new_err {
        Some(message) => message.to_owned(),
        None => cause.to_owned(),
    }
}

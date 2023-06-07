use super::errors::LogDetailsError;
use super::types::{LogType, PartialLogDetails};
use lazy_regex::regex;

pub fn destructure_log_id(id: String) -> Result<PartialLogDetails, LogDetailsError> {
    if id.len() != 6 {
        return Err(LogDetailsError::InvalidLength(id.len()));
    }

    let hex_chars = regex!("^([0-9A-F]+)$");
    if !hex_chars.is_match(&id) {
        return Err(LogDetailsError::InvalidCharacters);
    }

    let chars_crate = id.chars().take(1).collect::<String>();
    let chars_module = id.chars().skip(1).take(2).collect::<String>();
    let chars_type = id.chars().skip(3).take(1).collect::<String>();
    let chars_index = id.chars().skip(4).take(2).collect::<String>();

    let log_type = match chars_type.as_str() {
        "1" => LogType::InfoLog,
        "2" => LogType::WarningLog,
        "3" => LogType::ErrorLog,
        _ => return Err(LogDetailsError::InvalidLogType),
    };

    let log_index = u8::from_str_radix(&chars_index, 16).unwrap();

    Ok(PartialLogDetails {
        id,
        idx_crate: chars_crate,
        idx_mod: chars_module,
        log_type,
        log_index,
    })
}

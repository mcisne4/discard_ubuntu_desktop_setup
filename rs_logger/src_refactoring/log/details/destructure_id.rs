use super::SourceError;
use lazy_regex::regex;

pub fn destructure_id(id: String) -> Result<(String, String, String, LogType, u8), SourceError> {
    if id.len() != 6 {
        return Err(SourceError::InvalidLength(id.len()));
    }

    let hex_chars = regex!("^([0-9A-F]+)$");
    if !hex_chars.is_match(&id) {
        return Err(SourceError::InvalidCharacters);
    }

    let chars_crate = id.chars().take(1).collect::<String>();
    let chars_module = id.chars().skip(1).take(2).collect::<String>();
    let chars_type = id.chars().skip(3).take(1).collect::<String>();
    let chars_index = id.chars().skip(4).take(2).collect::<String>();

    let mut log_type = LogType::INFO;
    match chars_type.as_str() {
        "1" => log_type = LogType::INFO,
        "2" => log_type = LogType::WARNING,
        "3" => log_type = LogType::ERROR,
        _ => return Err(SourceError::ParseLogTypeError),
    }

    let mut log_index = u8::from_str_radix(&chars_index, 16).unwrap();

    Ok((id, chars_crate, chars_module, log_type, log_index))
}

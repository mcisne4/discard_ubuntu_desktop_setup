use lazy_regex::regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogDetailsError {
    #[error("The provided Log ID is not of appropriate length: {0}/6 characters found")]
    InvalidLength(usize),

    #[error("The provided Log ID contains invalid characters")]
    InvalidCharacters,

    #[error("Could not convert the '{0}' characters to a number")]
    CharToU8(String),

    #[error("The provided Log ID contains an invalid log type")]
    InvalidLogType,

    #[error("Could not find the location of the provided Log ID")]
    InvalidModPath,
}

/// Details of a log entry from a *Log ID*
///
/// # Fields:
/// - `id` : `String` - The *Log ID*
/// - `log_location` : `String` - The module path of where the log is used
/// - `log_type` : `String` - The log type for the log entry
///   - Available types are:
///     - `INFO`
///     - `WARN`
///     - `ERROR`
/// - `log_index` : `u8` - The index of the log entry in its corresponding log group
#[derive(Debug)]
pub struct LogDetails {
    pub id: String,
    pub log_location: String,
    pub log_type: String,
    pub log_index: u8,
}

pub struct PartialLogDetails {
    pub id: String,
    pub crate_idx: u8,
    pub mod_idx: u8,
    pub log_type: String,
    pub log_index: u8,
}
impl PartialLogDetails {
    pub fn from_log_id(log_id: &str) -> Result<Self, LogDetailsError> {
        if log_id.len() != 6 {
            return Err(LogDetailsError::InvalidLength(log_id.len()));
        }

        let hex_chars = regex!("^([0-9A-F]+)$");
        if !hex_chars.is_match(&log_id) {
            return Err(LogDetailsError::InvalidCharacters);
        }

        let chars_crate = log_id.chars().take(1).collect::<String>();
        let crate_idx = u8::from_str_radix(&chars_crate, 16)
            .map_err(|_| LogDetailsError::CharToU8(String::from("crate index")))?;

        let chars_module = log_id.chars().skip(1).take(2).collect::<String>();
        let mod_idx = u8::from_str_radix(&chars_module, 16)
            .map_err(|_| LogDetailsError::CharToU8(String::from("module index")))?;

        let chars_type = log_id.chars().skip(3).take(1).collect::<String>();
        let log_type = match chars_type.as_str() {
            "1" => String::from("INFO"),
            "2" => String::from("WARN"),
            "3" => String::from("ERROR"),
            _ => return Err(LogDetailsError::InvalidLogType),
        };

        let chars_index = log_id.chars().skip(4).take(2).collect::<String>();
        let log_index = u8::from_str_radix(&chars_index, 16)
            .map_err(|_| LogDetailsError::CharToU8(String::from("log index")))?;

        Ok(Self {
            id: log_id.to_owned(),
            crate_idx,
            mod_idx,
            log_type,
            log_index,
        })
    }

    pub fn add_src(self, src: &str) -> LogDetails {
        LogDetails {
            id: self.id,
            log_location: src.to_owned(),
            log_type: self.log_type,
            log_index: self.log_index,
        }
    }
}

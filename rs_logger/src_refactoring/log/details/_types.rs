/// Details of a Log ID
///
/// # Properties
///
/// * `id: String` - The Log ID
/// * `in_crate: Sting` - The crate where the log is used
/// * `log_use_path: String` - The path to where the log is used
/// * `log_type: LogType` - The type of log
/// * `log_index: u8` - The log index
#[derive(Debug)]
pub struct LogDetails {
    pub id: String,
    pub in_crate: String,
    pub log_use_path: String,
    pub log_index: u8,
    pub log_type: LogType,
}

/// Represents the type of log:
/// INFO, WARN, or ERROR
///
/// # Example:
/// ```
/// let info_log = LogType::INFO;
/// let expected_value = String::from("INFO");
/// assert_eq!(info_log.to_string(), expected_value);
///
/// let warn_log = LogType::WARNING;
/// let expected_value = String::from("WARN");
/// assert_eq!(warn_log.to_string(), expected_value);
///
/// let error_log = LogType::ERROR;
/// let expected_value = String::from("ERROR");
/// assert_eq!(error_log.to_string(), expected_value);
/// ```
#[derive(Debug)]
pub enum LogType {
    INFO,
    WARNING,
    ERROR,
}
impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INFO => write!(f, "INFO"),
            Self::WARNING => write!(f, "WARN"),
            Self::ERROR => write!(f, "ERROR"),
        }
    }
}

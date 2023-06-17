/// The type of log: **Info**, **Warning**, or **Error**
///
/// # Variants
/// - `LogType::InfoLog` - An **INFO** log
/// - `LogType::WarningLog` - A **WARN** log
/// - `LogType::ErrorLog` - An **ERROR** log
///
/// ## Example:
/// ```
/// let info_log =  LogType::InfoLog;
/// let warn_log =  LogType::WarningLog;
/// let error_log = LogType::ErrorLog;
/// ```
///
/// # Methods
/// - `to_string()` - Converts the `LogType` variant to a `String`
///
/// ## Example
/// ```
/// // Info Log
/// let info_log = LogType::InfoLog.to_string();
/// assert_eq!(info_log.as_str(), "INFO");
///
/// // Warning Log
/// let warn_log = LogType::WarningLog.to_string();
/// assert_eq!(warn_log.as_str(), "WARN");
///
/// // Error Log
/// let err_log = LogType::ErrorLog.to_string();
/// assert_eq!(err_log.as_str(), "ERROR");
/// ```
#[derive(Debug)]
pub enum LogType {
    InfoLog,
    WarningLog,
    ErrorLog,
}
impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InfoLog => write!(f, "INFO"),
            Self::WarningLog => write!(f, "WARN"),
            Self::ErrorLog => write!(f, "ERROR"),
        }
    }
}

/// Details about a Log ID
///
/// # Fields
/// Field | Type | Description
/// - | - | -
/// `id` | `String` | The Log ID
/// `mod_path` | `String` | The module path to where the log is used
/// `log_type` | `LogType` | The type of log
/// `log_index` | `u8` | The log index
///
/// # Example
/// ```
/// let details = rs_logger::log_details("0003F3");
///
/// // Log ID
/// let id = details.id;
/// assert_eq!(id.as_str(), "0003F3");
///
/// // Module Path
/// let mod_path = details.mod_path;
/// assert_eq!(mod_path.as_str(), "rs_logger::example");
///
/// // Log Type
/// let log_type = details.log_type;
/// let log_type = log_type.to_string();
/// assert_eq!(log_type.as_str(), "ERROR");
///
/// // Log Index
/// let log_idx = details.log_index;
/// assert_eq!(log_idx, 243_u8);
/// ```
#[derive(Debug)]
pub struct LogDetails {
    pub id: String,
    pub mod_path: String,
    pub log_type: LogType,
    pub log_index: u8,
}

pub struct PartialLogDetails {
    pub id: String,
    pub idx_crate: String,
    pub idx_mod: String,
    pub log_type: LogType,
    pub log_index: u8,
}

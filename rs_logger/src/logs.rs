mod log_details;
mod logger;

use log_details::{LogDetails, LogDetailsError, PartialLogDetails};
use logger::Logger;

/// A collection of available log variants
///
/// # Methods:
/// - `as_logger()` - Extracts a `Logger` from a log variant
/// - `get_log_source(log_id: &src)` - Get details of a log from a *Log ID*
///
/// # Example:
/// ```
/// use rs_logger::Logs;
///
/// // Logger
/// let logger = Logs::Test.as_logger();
/// logger.log_info("Adding an 'INFO' log");
///
/// // Log Details
/// match Logs::get_log_source("0003F3") {
///   Ok(details) => {
///     println!("Log Type: '{}'", details.log_type);
///   }
///   Err(e) => println!("{}", e.to_string()),
/// }
///
/// ```
pub enum Logs {
    Test,
}
impl Logs {
    /// Creates a **Logger** to write logs into a log file
    ///
    /// # Example:
    /// ```
    /// use rs_logger::Logs;
    ///
    /// let logger = Logs::Test.as_logger();
    ///
    /// logger.log_info("Inserting an information log");
    /// logger.log_warn("Inserting a warning log");
    /// logger.log_error("Inserting an error log");
    /// ```
    pub fn as_logger(&self) -> Logger {
        match self {
            Self::Test => Logger::new(0, 0),
        }
    }

    /// Get details of a log from a *Log ID*
    ///
    /// # Arguments:
    ///
    /// - `log_id` : `&str` - A string of the hexadecimal *Log ID*
    ///
    /// The *Log ID* is the 6 character string found in the header of a log entry
    ///
    /// ## Log Entry Examples:
    /// - For the Log Entries:
    ///   - `[16:48:36 INFO  6A3104] Log details lorem ipsum`
    ///   - `[08:16:01 WARN  202236] Log details lorem ipsum`
    ///   - `[23:59:59 ERROR B9F3D7] Log details lorem ipsum`
    /// * The respective *Log IDs* are:
    ///   - `6A3104`
    ///   - `202236`
    ///   - `B9F3D7`
    ///
    /// # Example:
    /// ```
    /// use rs_logger::Log;
    /// let details = Log::get_log_details("0003F3");
    ///
    /// // Log ID
    /// let id = details.id;
    /// assert_eq!(id, "0003F3".to_owned());
    ///
    /// let log_location = details.log_location;
    /// assert_eq!(log_location, "rs_logger::example".to_owned());
    ///
    /// let log_type = details.log_type;
    /// assert_eq!(log_type, "ERROR".to_owned());
    ///
    /// let log_index = details.log_index;
    /// assert_eq!(log_index, 243_u8);
    /// ```
    pub fn get_log_source(log_id: &str) -> Result<LogDetails, LogDetailsError> {
        let pre_details = PartialLogDetails::from_log_id(log_id)?;

        let crate_mod: u16 =
            u16::from(pre_details.crate_idx) * 100 + u16::from(pre_details.mod_idx);

        let details = match crate_mod {
            0 => pre_details.add_src("rs_logger::example"),

            _ => return Err(LogDetailsError::InvalidModPath),
        };

        Ok(details)
    }
}

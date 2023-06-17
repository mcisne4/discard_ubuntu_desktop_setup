use log::{error as error_macro, info as info_macro, warn as warn_macro};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReLoggerErrors {
    #[error("Improper amount of crate and mod characters provided")]
    InvalidCharLength,

    #[error("")]
}

pub struct ReLogger(u8, u8);
impl ReLogger {
    pub fn new(crate_idx: u8, mod_idx: u8) -> Self {
        Self(crate_idx, mod_idx)
    }

    pub fn from_str(crate_and_mod_chars: &str) -> Result<Self, ReLoggerErrors> {
        if crate_and_mod_chars.len() != 3 {
            return Err(ReLoggerErrors::InvalidCharLength);
        }
    }
}

/// Used to write data to a log file
///
/// # Methods:
/// - `log_info(content: S)` - Adds an *INFO* entry to the log file
/// - `log_warn(content: S)` - Adds a *WARN* entry to the log file
/// - `log_error(content: S)` - Adds an *ERROR* entry to the log file
///
/// # Example:
/// ```
/// use rs_logger::Logs;
///
/// let logger = Logs::Test.as_logger();
///
/// // Add an 'INFO' entry
/// logger.log_info("Inserting an information log");
///
/// // Add a 'WARN' entry
/// logger.log_warn("Inserting a warning log");
///
/// // Add an 'ERROR' entry
/// logger.log_error("Inserting an error log");
/// ```
pub struct Logger {
    pub crate_idx: u8,
    pub mod_idx: u8,
    log_count: u8,
}
impl Logger {
    pub fn new(crate_idx: u8, mod_idx: u8) -> Self {
        Self {
            crate_idx,
            mod_idx,
            log_count: 0,
        }
    }

    fn get_log_id(&self, log_type: u8) -> String {
        let mut log_id = String::new();

        log_id += format!("{:X}", self.crate_idx & 15).as_str();
        log_id += format!("{:02X}", self.mod_idx).as_str();
        log_id += format!("{:X}", log_type % 15).as_str();
        log_id += format!("{:02X}", self.log_count).as_str();

        log_id
    }

    /// Adds an 'INFO' entry to the log file
    ///
    /// # Arguments:
    /// - `log_type` : `String`, `&str`, `&String` - A string of the *Log ID*
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
    ///
    /// let logger = Log::Test.as_logger();
    ///
    /// logger.log_info("Adding an information entry");
    /// logger.log_info(String::from("Adding another entry"));
    /// ```
    pub fn log_info<S>(&mut self, idx: u8, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        self.log_count += 1;
        info_macro!("{}] {}", self.get_log_id(1), content);
    }

    /// Adds an 'WARN' entry to the log file
    ///
    /// # Arguments:
    /// - `log_type` : `String`, `&str`, `&String` - A string of the *Log ID*
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
    ///
    /// let logger = Log::Test.as_logger();
    ///
    /// logger.log_warn("Adding a warning entry");
    /// logger.log_warn(String::from("Adding another entry"));
    /// ```
    pub fn log_warn<S>(&mut self, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        self.log_count += 1;
        warn_macro!("{}] {}", self.get_log_id(2), content);
    }

    /// Adds an 'ERROR' entry to the log file
    ///
    /// # Arguments:
    /// - `log_type` : `String`, `&str`, `&String` - A string of the *Log ID*
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
    ///
    /// let logger = Log::Test.as_logger();
    ///
    /// logger.log_error("Adding an error entry");
    /// logger.log_error(String::from("Adding another entry"));
    /// ```
    pub fn log_error<S>(&mut self, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        self.log_count += 1;
        error_macro!("{}] {}", self.get_log_id(1), content);
    }
}

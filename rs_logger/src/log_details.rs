mod destructure_log_id;
mod errors;
mod types;

use self::{destructure_log_id::destructure_log_id, errors::LogDetailsError, types::LogDetails};

/// Returns details about a provided *Log ID*
///
/// # Arguments:
///
/// - `log_id` : `String`,`&str`, `&String` - A string of the hexadecimal *Log ID*
///
/// The *Log ID* is the 6 character string found in the header of a log entry.
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
pub fn log_details<S>(log_id: S) -> Result<LogDetails, LogDetailsError>
where
    S: AsRef<str> + std::fmt::Display,
{
    let id = format!("{}", log_id);
    let partial_details = destructure_log_id(id)?;

    let crate_and_module = partial_details.idx_crate + &partial_details.idx_mod;
    let mut mod_path = String::new();

    mod_path += match crate_and_module.as_str() {
        "000" => "rs_logger::example",
        "101" => "rs_logger::init::configure",
        "102" => "rs_logger::init::remove_old_logs",
        _ => return Err(LogDetailsError::InvalidModPath),
    };

    Ok(LogDetails {
        id: partial_details.id,
        mod_path,
        log_type: partial_details.log_type,
        log_index: partial_details.log_index,
    })
}

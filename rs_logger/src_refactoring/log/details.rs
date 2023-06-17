// ============= LOG FORMAT ============= //
// [crate][module][log_type][log_index]
// [  F  ][  FF  ][   F    ][   FF    ]
//
// [16:23:48 INFO  10A102] Lorem ipsum
// [16:23:48 WARN  D1F20F] Lorem ipsum
// [16:23:48 ERROR 6083AF] Lorem ipsum
//
// ====================================== //

mod errors;
pub use errors::SourceError;
mod destructure_id;
use destructure_id::destructure_id;
mod _types;
pub use _types::Testing;

/// Returns details about a provided Log ID
///
/// # Arguments:
///
/// * `log_id` - A string of the hexadecimal log ID
///   - Accepts `String`, `&str`, and `&String` values
///
/// ## Log ID:
/// The Log ID is the 6 character string found in the header
/// of a log entry.
///
/// ### Log Entry Examples:
/// * For Log Entries:
///   - `[16:48:36 INFO  6A3104] Log details lorem ipsum`
///   - `[08:16:01 WARN  202236] Log details lorem ipsum`
///   - `[23:59:59 ERROR B9F3D7] Log details lorem ipsum`
/// * The respective Log IDs are:
///   - `6A3104`
///   - `202236`
///   - `B9F3D7`
///
/// # Example:
/// Basic Usage
/// ```
/// match log_details("6A3104") {
///     Ok(details: LogDetails) => {
///         let id = details.id;
///         assert_eq!(id, String::from("6A3104"));
///
///         let in_crate = details.in_crate;
///         assert_eq!(in_crate, String::from("rs_example"));
///
///         let log_use_path = details.log_use_path;
///         assert_eq!(log_use_path, String::from("rs_example::mod1::mod2"));
///
///         let log_type = details.log_type;
///         assert_eq!(log_type, String::from("LOG"));
///
///         let log_index = details.log_index;
///         assert_eq!(log_index, 4_u8);
///     },
///     Err(e: SourceErrors) => println!("{}", e.to_string()),
/// }
pub fn log_details<S>(log_id: S) -> Result<LogDetails, SourceError>
where
    S: AsRef<str> + std::fmt::Display,
{
    let (id, crate_str, mod_str, log_type, log_index) = destructure_id(format!("{}", log_id))?;

    let crate_and_module = crate_str + &mod_str;
    let mut log_use_path = String::new();

    log_use_path += match crate_and_module.as_str() {
        "101" => "rs_logger::init",
        _ => return Err(SourceError::ParseLogUsePath),
    };

    let in_crate = log_use_path.split("::").take(1).collect::<String>();

    Ok(LogDetails {
        id,
        in_crate,
        log_use_path,
        log_type,
        log_index,
    })
}

fn test() {
    match log_details("6A3104") {
        Ok(details) => {
            let id = details.id;
            assert_eq!(id, String::from("6A3104"));

            let in_crate = details.in_crate;
            assert_eq!(in_crate, String::from("rs_example"));

            let log_use_path = details.log_use_path;
            assert_eq!(log_use_path, String::from("rs_example::mod1::mod2"));

            let log_type = details.log_type;
            assert_eq!(log_type, String::from("LOG"));

            let log_index = details.log_index;
            assert_eq!(log_index, 4_u8);
        }
        Err(e) => println!("{}", e.to_string()),
    }
}

use hifitime::prelude::{Epoch, Format, Formatter};
use std::str::FromStr;

pub fn timestamp() -> Result<String, String> {
    let now = Epoch::now()
        .map_err(|e| format!("ERROR: Unable to get current time:\n  {}", e.to_string()))?;

    let timestamp_format_str = "%Y-%m-%dT%H:%M:%S";
    let timestamp_format = Format::from_str(timestamp_format_str).map_err(|_e| {
        format!(
            "ERROR: Unable to format time to '{}'\n  {}",
            timestamp_format_str, "hifitime::ParsingError"
        )
    })?;

    let timestamp = Formatter::new(now, timestamp_format);
    Ok(format!("{}Z", timestamp))
}

pub fn get_timestamp() -> String {
    match timestamp() {
        Ok(time_stamp) => time_stamp,
        Err(_e) => String::from("0000-00-00T00:00:00Z"),
    }
}

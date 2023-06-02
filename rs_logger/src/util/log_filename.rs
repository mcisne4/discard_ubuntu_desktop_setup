use hifitime::prelude::{Epoch, Format, Formatter};
use std::str::FromStr;

pub fn log_filename() -> Result<String, String> {
    let current_epoch = Epoch::now()
        .map_err(|e| format!("ERROR: Unable to get current time:\n  {}", e.to_string()))?;

    let fmt = Format::from_str("%Y%m%d")
        .map_err(|_e| format!("ERROR: Unable to format time:\n  {}", "ParsingError"))?;

    let name = Formatter::new(current_epoch, fmt);
    let filename = format!("{}_info.log", name);
    Ok(filename)
}

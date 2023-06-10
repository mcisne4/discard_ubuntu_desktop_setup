use super::errors::InitError;
use hifitime::prelude::{Epoch, Format, Formatter};
use std::str::FromStr;

pub fn generate_filename() -> Result<String, InitError> {
    let current_epoch = Epoch::now().map_err(|e| InitError::FilenameEpoch(e))?;

    let fmt = Format::from_str("%Y%m%d").map_err(|_e| InitError::FilenameTimeFormat)?;

    let name = Formatter::new(current_epoch, fmt);
    let filename = format!("{}_details.log", name);
    Ok(filename)
}

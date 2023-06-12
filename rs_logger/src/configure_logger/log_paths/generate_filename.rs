use super::super::errors::ConfigErrors;
use hifitime::prelude::{Epoch, Format, Formatter};
use std::str::FromStr;

pub fn generate_filename() -> Result<String, ConfigErrors> {
    let current_epoch = Epoch::now().map_err(|e| ConfigErrors::FilenameEpoch(e))?;

    let fmt = Format::from_str("%Y%m%d").map_err(|_e| ConfigErrors::FilenameTimeFormat)?;

    let name = Formatter::new(current_epoch, fmt);
    let filename = format!("{}_details.log", name);
    Ok(filename)
}

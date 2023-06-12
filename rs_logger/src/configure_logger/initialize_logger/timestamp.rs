use hifitime::prelude::{Epoch, Format, Formatter};
use std::str::FromStr;

pub fn timestamp() -> String {
    let default_timestamp = String::from("00:00:00");

    match Epoch::now() {
        Err(_) => return default_timestamp,
        Ok(epoch) => match Format::from_str("%H:%M:%S") {
            Err(_) => default_timestamp,
            Ok(timestamp_format) => {
                let timestamp = Formatter::new(epoch, timestamp_format);
                format!("{}", timestamp)
            }
        },
    }
}

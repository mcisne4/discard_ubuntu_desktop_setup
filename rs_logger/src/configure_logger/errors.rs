use hifitime::Errors as HifitimeError;
use log::SetLoggerError;
use std::io::Error as IoError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigErrors {
    #[error("DIRECTORY ERROR: The provided path is not a valid directory\n\tPath: '{0}'")]
    LogDirectory(PathBuf),

    #[error("FILENAME ERROR: Unable to get the current time:\n\t{0}")]
    FilenameEpoch(HifitimeError),

    #[error("FILENAME ERROR: Unable to format time")]
    FilenameTimeFormat,

    #[error("CONFIGURATION ERROR: Log file error:\n\t{0}")]
    ConfigLogFile(IoError),

    #[error("CONFIGURATION ERROR: Log dispatch error:\n\t{0}")]
    ConfigDispatch(SetLoggerError),
}

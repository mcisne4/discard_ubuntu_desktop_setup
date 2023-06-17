use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogDetailsError {
    #[error("The provided Log ID is not of appropriate length: {0}/6 characters found")]
    InvalidLength(usize),

    #[error("The provided Log ID contains invalid characters")]
    InvalidCharacters,

    #[error("The provided Log ID contains an invalid log type")]
    InvalidLogType,

    #[error("The provided Log ID contains an invalid module path")]
    InvalidModPath,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("provided log ID is not of appropriate length: {0}/6 characters found")]
    InvalidLength(usize),
    #[error("provided log ID contains invalid characters")]
    InvalidCharacters,
    #[error("could not parse the log type")]
    ParseLogTypeError,
    #[error("could not parse the specified crate")]
    ParseCrateError,
    #[error("could not parse the specified module")]
    ParseModuleError,
    #[error("could not parse the path where the log is used")]
    ParseLogUsePath,
}

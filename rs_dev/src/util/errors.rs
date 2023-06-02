use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("A logging error caused by: '{0}'")]
    AnError(String),
}

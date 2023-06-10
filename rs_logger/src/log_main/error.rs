use crate::log_functions::error_log;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorLog {
    #[error("Test Error Log")]
    Id000301,
}
impl ErrorLog {
    pub fn log(self) -> String {
        match self {
            Self::Id000301 => {
                error_log("000301", self.to_string());
                self.to_string()
            }
        }
    }
}

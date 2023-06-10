use crate::log_functions::warning_log;

pub enum WarningLog {
    Id000201,
}
impl WarningLog {
    pub fn log(self) {
        match self {
            Self::Id000201 => warning_log("000201", "Test Warning Log"),
        }
    }
}

mod log_details;
pub use log_details::log_details;

mod init;
pub use init::init;

mod log_functions;

mod log_main;
pub use log_main::{ErrorLog, InfoLog, WarningLog};

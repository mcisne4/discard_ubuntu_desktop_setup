use log::{error as log_error, info as log_info, warn as log_warn};

pub struct Logger {
    pub crate_idx: u8,
    pub mod_idx: u8,
    pub mod_path: String,
    log_count: u8,
}
impl Logger {
    pub fn new(crate_idx: u8, mod_idx: u8, mod_path: &str) -> Self {
        Self {
            crate_idx,
            mod_idx,
            mod_path: mod_path.to_owned(),
            log_count: 0,
        }
    }

    fn get_log_id(&self) -> String {
        format!(
            "{}{}{}{}",
            self.crate_idx, self.mod_idx, 1_u8, self.log_count
        )
    }

    pub fn info<S>(&mut self, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        log_info!("{}] {}", self.get_log_id(), content);
        self.log_count += 1;
    }

    pub fn warn<S>(&mut self, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        log_warn!("{}] {}", self.get_log_id(), content);
        self.log_count += 1;
    }

    pub fn error<S>(&mut self, content: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        log_error!("{}] {}", self.get_log_id(), content);
        self.log_count += 1;
    }
}

mod logger;
use logger::Logger;

pub enum Logs {
    Test,
}
impl Logs {
    pub fn as_logger(&self) -> Logger {
        match self {
            Self::Test => Logger::new(0, 0, "rs_logger::example"),
        }
    }

    pub fn find(log_id: &str) -> String {
        let id = log_id.chars().take(3).collect::<String>();
        match id.as_str() {
            "000" => String::from("rs_logger::example"),
            _ => String::from(""),
        }
    }
}

fn dev() {
    let mut logger = Logs::Test.as_logger();
    logger.info("content");

    Logs::find("203");
}

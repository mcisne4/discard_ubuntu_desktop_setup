// use crate::
// mod mods;
// mod util;

// pub use mods::rs_logger::{dev_rs_logger, LoggerMode};

// mod lib;
use rs_dev::rs_logger::{dev_rs_logger, LoggerMode};

fn main() {
    println!("=======================================");
    println!("========= Hello from 'rs_dev' =========");
    println!("=======================================\n");

    dev_rs_logger(LoggerMode::InitLogger);

    // mods::rs_shell::dev(false);
    // mods::rs_sqlite_db::dev(false);
}

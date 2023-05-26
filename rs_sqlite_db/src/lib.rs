use std::path::PathBuf;

mod db;
use db::connect;

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let conn = connect(db_dir)?;

    Ok(())
}

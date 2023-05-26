use std::path::PathBuf;

mod db;
use db::{connect, table_exists};

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let conn = connect(db_dir)?;

    let table_exists = table_exists(&conn)?;
    println!("TABLE EXISTS: {}", table_exists);

    Ok(())
}

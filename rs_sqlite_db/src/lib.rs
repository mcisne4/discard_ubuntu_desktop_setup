use std::path::PathBuf;

mod db;
use db::{check_table_columns, connect, table_exists};

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let conn = connect(db_dir)?;

    let table_exists = table_exists(&conn)?;
    println!("TABLE EXISTS: {}", table_exists);

    if table_exists {
        let columns_are_valid = check_table_columns(&conn)?;
        println!("COLUMNS ARE VALID: {}", columns_are_valid);
    }

    Ok(())
}

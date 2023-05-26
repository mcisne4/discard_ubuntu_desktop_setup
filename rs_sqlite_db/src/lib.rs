use std::path::PathBuf;

mod db;
use db::{check_table_columns, check_table_exists, check_table_row_count, connect, table_drop};
use rusqlite::Connection;

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let conn = connect(db_dir)?;

    // --- Validate Existing DB --- //
    let drop_table = should_drop_table(&conn)?;
    println!("SHOULD DROP TABLE?: {}", drop_table);
    if drop_table {
        match table_drop(&conn) {
            Ok(_) => println!("TABLE DROPPED"),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

fn should_drop_table(conn: &Connection) -> Result<bool, String> {
    println!("--- VERIFYING DATABASE ---");

    // --- Table Exists --- //
    let table_exists = check_table_exists(&conn)?;
    println!("TABLE EXISTS?: {}", table_exists);
    if !table_exists {
        return Ok(false);
    }

    // --- Check Columns -- //
    let columns_are_valid = check_table_columns(&conn)?;
    println!("COLUMNS ARE VALID?: {}", columns_are_valid);
    if !columns_are_valid {
        return Ok(true);
    }

    // --- Check Row Count --- //
    let row_count_is_valid = check_table_row_count(&conn)?;
    println!("ROW COUNT IS VALID?: {}", row_count_is_valid);
    if !row_count_is_valid {
        return Ok(true);
    }

    Ok(false)
}

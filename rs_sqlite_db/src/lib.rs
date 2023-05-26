use rusqlite::Connection;
use std::path::PathBuf;

mod db;
use db::{
    check_table_columns, check_table_exists, check_table_row_count, connect, create_table,
    insert_rows, table_drop,
};
mod row_data;
use row_data::get_row_data;

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let conn = connect(db_dir)?;

    // --- Validate Existing DB --- //
    let (drop_table, make_table) = validate_existing(&conn)?;

    // --- Drop Existing DB --- //
    println!("SHOULD DROP TABLE?: {}", drop_table);
    match drop_table {
        true => {
            table_drop(&conn)?;
            println!("TABLE DROPPED");
        }
        false => (),
    }

    // --- Create DB --- //
    println!("SHOULD CREATE TABLE?: {}", make_table);
    match make_table {
        true => {
            create_table(&conn)?;
            println!("TABLE CREATED");

            let rows = get_row_data();
            insert_rows(&conn, rows)?;
            println!("ROWS ADDED");
        }
        false => (),
    }

    // --- Drop Connection --- //
    match conn.close() {
        Ok(_) => println!("Connection closed"),
        Err((_conn, e)) => print!("Unable to close connection:\n  {}", e.to_string()),
    }

    Ok(())
}

fn validate_existing(conn: &Connection) -> Result<(bool, bool), String> {
    println!("--- VERIFYING DATABASE ---");

    // --- Table Exists --- //
    let table_exists = check_table_exists(&conn)?;
    println!("TABLE EXISTS?: {}", table_exists);
    if !table_exists {
        return Ok((false, true));
    }

    // --- Check Columns -- //
    let columns_are_valid = check_table_columns(&conn)?;
    println!("COLUMNS ARE VALID?: {}", columns_are_valid);
    if !columns_are_valid {
        return Ok((true, true));
    }

    // --- Check Row Count --- //
    let row_count_is_valid = check_table_row_count(&conn)?;
    println!("ROW COUNT IS VALID?: {}", row_count_is_valid);
    if !row_count_is_valid {
        return Ok((true, true));
    }

    Ok((false, false))
}

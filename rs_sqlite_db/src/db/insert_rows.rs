use row_data::RowData;
use rusqlite::Connection;

use crate::row_data;

pub fn insert_rows(conn: &Connection, rows: Vec<RowData>) -> Result<(), String> {
    let mut statement = conn
        .prepare(
            "INSERT INTO command_list
    (command, title, description, keywords, use_count)
    VALUES (?1, ?2, ?3, ?4, ?5);",
        )
        .map_err(|e| e.to_string())?;

    for row_data in rows {
        let args = row_data.as_args();
        statement.execute(args).map_err(|e| e.to_string())?;
    }

    Ok(())
}

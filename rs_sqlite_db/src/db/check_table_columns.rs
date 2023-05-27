use rusqlite::Connection;

pub fn check_table_columns(conn: &Connection) -> Result<bool, String> {
    let statement = conn
        .prepare("SELECT * FROM command_list")
        .map_err(|e| e.to_string())?;

    let db_cols = statement.column_names();
    let req_cols: Vec<&str> = vec!["command", "title", "description", "keywords", "use_count"];

    let cols_match = db_cols == req_cols;

    statement.finalize().map_err(|e| e.to_string())?;

    Ok(cols_match)
}

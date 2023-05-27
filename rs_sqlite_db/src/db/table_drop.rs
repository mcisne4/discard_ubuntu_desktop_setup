use rusqlite::Connection;

pub fn table_drop(conn: &Connection) -> Result<(), String> {
    conn.execute("DROP TABLE IF EXISTS command_list", [])
        .map_err(|e| e.to_string())?;

    Ok(())
}

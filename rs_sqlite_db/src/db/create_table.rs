use rusqlite::Connection;

pub fn create_table(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_list(
      command TEXT PRIMARY KEY,
      title TEXT NOT NULL,
      description TEXT NOT NULL,
      keywords TEXT NOT NULL,
      use_count INTEGER NOT NULL
  );",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

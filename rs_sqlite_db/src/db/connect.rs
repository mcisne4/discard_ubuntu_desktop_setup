use rusqlite::Connection;
use std::path::PathBuf;

pub fn connect(db_dir: PathBuf) -> Result<Connection, String> {
    let mut db_path = db_dir;
    db_path.push("cmd_list.db");

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    Ok(conn)
}

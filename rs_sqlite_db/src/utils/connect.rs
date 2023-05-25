// use std::env;
use std::path::PathBuf;

use rusqlite::Connection;

pub fn connect(db_path: PathBuf) -> Result<Connection, String> {
    let mut db_path = db_path;
    db_path.push("cmd_list.db");
    println!("DB_PATH:\n  {:?}", db_path);

    Connection::open(db_path).map_err(|e| e.to_string())
}

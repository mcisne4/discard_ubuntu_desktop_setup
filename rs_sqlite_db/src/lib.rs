use rusqlite::Connection;
use std::path::PathBuf;

mod row_data;

pub fn hello() {
    println!("Hello from 'rs_sqlite_db'");
}

pub fn init(db_dir: PathBuf) -> Result<(), String> {
    // --- Connection --- //
    let mut db_path = db_dir;
    db_path.push("cmd_list.db");
    println!("DB_PATH:\n  {:?}", db_path);

    let db = Connection::open(db_path).map_err(|e| e.to_string())?;

    db.execute(
        "CREATE TABLE commands(
            id INTEGER PRIMARY KEY,
            cmd TEXT NOT NULL,
            cmd_name TEXT NOT NULL,
            prev_used INTEGER NOT NULL,
            description TEXT NOT NULL,
            keywords TEXT NOT NULL
        );",
        (),
    )
    .map_err(|e| e.to_string())?;

    let rows = row_data::get_row_data();

    let stmt = db
        .prepare("INSERT INTO commands (id, cmd, cmd_name, prev_used, description, keywords) VALUES (?1, ?2, ?3, ?4, ?5, ?6);")
        .map_err(|e| e.to_string())?;

    for entry in rows {
        stmt.execute(params)
    }

    Ok(())
}

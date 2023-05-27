use rs_sqlite_db::init;
use std::env;

pub fn dev(run: bool) {
    if !run {
        return;
    }

    // --- Section Heading --- //
    println!("--------------------------------------------");
    println!("--- Developing for Crate: 'rs_sqlite_db' ---");
    println!("--------------------------------------------\n");

    // --- Dev DB Directory --- //
    let mut db_dir = env::current_dir().unwrap();
    db_dir.push("dev_db/");
    println!("SQLite Database Directory:\n  '{}'\n", &db_dir.display());

    // --- Initialize SQLite Database --- //
    println!("Initializing SQLite Database");
    match init(db_dir) {
        Ok(_) => println!("SQLite Database Initialization: Successfully\n"),
        Err(e) => println!("ERROR: SQLite Initialization\n  {}\n", e),
    }
}

use std::env;

use rs_shell;
use rs_sqlite_db;

fn main() {
    println!("Hello from 'rs_dev'");

    // === CRATE: 'rs_shell' === //

    rs_shell::hello();

    // // --- Get $SHELL --- //
    // match rs_shell::env::get_shell() {
    //     Ok(val) => println!("\nenv $SHELL: {}", val),
    //     Err(e) => println!("\nenv $SHELL ERROR: '{}'", e),
    // }

    // // --- Is Installed?: ZSH --- //
    // match rs_shell::is_installed::zsh() {
    //     Ok(is_installed) => println!("\nZSH is installed?: {}", is_installed),
    //     Err(e) => println!("\nZSH is installed Err?: {}", e),
    // }

    // // --- Is Installed?: BASH --- //
    // match rs_shell::is_installed::zsh() {
    //     Ok(is_installed) => println!("\nBASH is installed?: {}", is_installed),
    //     Err(e) => println!("\nBASH is installed Err?: {}", e),
    // }

    // === CRATE: 'rs_sqlite_db' === //

    rs_sqlite_db::hello();

    let mut db_dir = env::current_dir().unwrap();
    db_dir.push("db");

    match rs_sqlite_db::init(db_dir) {
        Ok(_) => println!("Initialization completed successfully"),
        Err(e) => println!("Initialization Error:\n  {:?}", e),
    }

    // match rs_sqlite_db::connect(db_dir) {
    //     Ok(db) => {
    //         println!("Connection Successful");
    //         match db.close() {
    //             Ok(_) => println!("Connection closed successfully"),
    //             Err(e) => println!("Error closing connection:\n  {:?}", e),
    //         }
    //     }
    //     Err(e) => println!("Connection Error:\n  {}", e),
    // };

    println!("");
}

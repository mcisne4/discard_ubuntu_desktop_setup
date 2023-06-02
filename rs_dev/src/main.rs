mod mods;
mod util;

fn main() {
    println!("=======================================");
    println!("========= Hello from 'rs_dev' =========");
    println!("=======================================\n");

    mods::rs_logger::dev(true);
    mods::rs_shell::dev(false);
    mods::rs_sqlite_db::dev(false);
}

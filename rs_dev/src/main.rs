mod mods;

fn main() {
    println!("=======================================");
    println!("========= Hello from 'rs_dev' =========");
    println!("=======================================\n");

    mods::rs_logs::dev(true);
    mods::rs_shell::dev(false);
    mods::rs_sqlite_db::dev(false);
}

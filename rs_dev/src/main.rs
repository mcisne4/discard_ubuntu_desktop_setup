mod mods;

fn main() {
    println!("=======================================");
    println!("========= Hello from 'rs_dev' =========");
    println!("=======================================\n");

    mods::rs_shell::dev(true);
    mods::rs_sqlite_db::dev(true);
}

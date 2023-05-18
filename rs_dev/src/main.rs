use rs_shell;

fn main() {
    println!("Hello, world! from 'rs_dev'");
    rs_shell::hello();

    // match rs_shell::shell::check_shell() {
    //     Ok(res) => println!("SHELL:\n  {:?}", res),
    //     Err(e) => println!("SHELL ERROR:\n  {:?}", e),
    // }

    // match rs_shell::shell::get_shell() {
    //     Ok(val) => println!("\nSHELL: {}\n", val),
    //     Err(e) => println!("\nSHELL ERROR: '{}'\n", e),
    // }

    match rs_shell::env::get_shell() {
        Ok(val) => println!("\nSHELL: {}\n", val),
        Err(e) => println!("\nSHELL ERROR: '{}'\n", e),
    }
}

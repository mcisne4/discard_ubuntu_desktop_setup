use rs_shell;

fn main() {
    println!("Hello, world! from 'rs_dev'");
    rs_shell::hello();

    // --- Get $SHELL --- //
    match rs_shell::env::get_shell() {
        Ok(val) => println!("\nenv $SHELL: {}", val),
        Err(e) => println!("\nenv $SHELL ERROR: '{}'", e),
    }

    // --- Is Installed?: ZSH --- //
    match rs_shell::is_installed::zsh() {
        Ok(is_installed) => println!("\nZSH is installed?: {}", is_installed),
        Err(e) => println!("\nZSH is installed Err?: {}", e),
    }

    // --- Is Installed?: BASH --- //
    match rs_shell::is_installed::zsh() {
        Ok(is_installed) => println!("\nBASH is installed?: {}", is_installed),
        Err(e) => println!("\nBASH is installed Err?: {}", e),
    }

    println!("");
}

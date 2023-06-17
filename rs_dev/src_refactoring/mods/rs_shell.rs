use rs_shell::{env, is_installed};

pub fn dev(run: bool) {
    if !run {
        return;
    }

    // --- Section Heading --- //
    println!("----------------------------------------");
    println!("--- Developing for Crate: 'rs_shell' ---");
    println!("----------------------------------------\n");

    // --- env $SHELL --- //
    match env::shell() {
        Ok(val) => println!("env $SHELL: {}\n", val),
        Err(e) => println!("env $SHELL ERROR: '{}'\n", e),
    }

    // --- Is Installed?: BASH --- //
    match is_installed::bash() {
        Ok(is_installed) => println!("Is 'bash' installed?: {}\n", is_installed),
        Err(e) => println!("ERROR: Is 'bash' installed?:\n  {}\n", e),
    }

    // --- Is Installed?: ZSH --- //
    match is_installed::zsh() {
        Ok(is_installed) => println!("Is 'zsh' installed?: {}\n", is_installed),
        Err(e) => println!("ERROR: Is 'zsh' installed?:\n  {}\n", e),
    }
}

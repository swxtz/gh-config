mod config;
mod logger;
mod utils;

use crate::config::os::verify_os;
use crate::utils::commands::run_command;

fn main() {
    verify_os();

    match run_command("git", ["config", "--global", "user.name", "Gustavo Mendonça"].to_vec()) {
        Ok(_) => println!("Deu certo"),
        Err(e) => eprint!("{}", e),
    }

    match run_command("git", ["config", "--global", "user.email", "teste@teste.com"].to_vec()) {
        Ok(_) => println!("Deu certo"),
        Err(e) => eprint!("{}", e),
    }

    println!("Hello, world!");
}

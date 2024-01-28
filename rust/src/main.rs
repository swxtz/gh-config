mod logger;
mod utils;

use crate::utils::commands::run_command;

fn main() {
    run_command("git", ["config", "--global", "user.name", "Gustavo Mendonça"].to_vec()).expect("Error");
    run_command("git", ["config", "--global", "user.email", "teste@teste.com"].to_vec()).expect("Error");

    println!("Hello, world!");
}

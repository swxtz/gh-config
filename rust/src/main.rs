mod logger;
mod utils;

use std::fs::File;
use std::path::Path;
use crate::utils::commands::run_command;
use crate::utils::downloader::download_font;

fn main() {
    run_command("git", ["config", "--global", "user.name", "Gustavo Mendonça"].to_vec()).expect("Error");
    run_command("git", ["config", "--global", "user.email", "dev.gustavomendona@protonmail.com"].to_vec()).expect("Error");

    download_font();
    println!("Hello, world!");
}





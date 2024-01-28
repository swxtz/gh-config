mod logger;
mod utils;

use crate::utils::commands::run_command;
use crate::utils::downloader::download_font;

fn main() {
    run_command(
        "git",
        ["config", "--global", "user.name", "Gustavo Mendonça"].to_vec(),
    )
    .expect("Error");
    run_command(
        "git",
        [
            "config",
            "--global",
            "user.email",
            "dev.gustavomendonca@protonmail.com",
        ]
        .to_vec(),
    )
    .expect("Error");

    download_font().expect("Erro ao baixar fonte");

    let os = std::env::consts::OS;
    if os == "windows" {
        run_command("explorer", ["."].to_vec()).expect("Error no OS");
    } else if os == "linux" {
        run_command("nautilus", ["."].to_vec()).expect("Error no OS");
    } else if os == "macos" {
        println!("Mac OS não é suportado ainda")
    }
}

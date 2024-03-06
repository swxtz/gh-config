mod utils;

use crate::utils::commands::run_command;
use crate::utils::downloader::download_font;
use webbrowser;

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

    webbrowser::open("https://github.com/cli/cli/releases/download/v2.45.0/gh_2.45.0_windows_amd64.msi").expect("Erro ao abrir navegador");
    webbrowser::open("https://download.jetbrains.com/idea/ideaIC-2023.3.4.exe?_ga=2.15497974.1590588974.1709763899-36386924.1709763899&_gl=1*my2gq5*_ga*MzYzODY5MjQuMTcwOTc2Mzg5OQ..*_ga_9J976DJZ68*MTcwOTc2NzY4OC4yLjAuMTcwOTc2NzY4OS4wLjAuMA..").expect("Erro ao abrir navegador");
    
    let os = std::env::consts::OS;
    if os == "windows" {
        run_command("explorer", ["."].to_vec()).expect("Error no OS");
    } else if os == "linux" {
        run_command("nautilus", ["."].to_vec()).expect("Error no OS");
    } else if os == "macos" {
        println!("Mac OS não é suportado ainda")
    }
}

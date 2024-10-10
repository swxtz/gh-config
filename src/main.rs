mod utils;

use crate::utils::commands::run_command;
use crate::utils::downloader::download_font;
use std::io;
use webbrowser;

fn main() {

    println!("
    =====================================================
    |                                                   |
    | Digite o numero do usuario para configurar o git  |
    |                                                   |
    | 1 - Gustavo                                       |
    | 2 - Sarah                                         |
    | 3 - Grazi                                         |
    | 4 - Rafael                                        |
    |                                                   |
    =====================================================
    ");

    println!("digite o numero do usuario:");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Falha ao ler linha");
    let num: i32 = user_input.trim().parse().expect("Por favor, digite um número válido");
    
    match num {
        1 => run_user_one(),
        2 => run_user_two(),
        3 => run_user_three(),
        4 => run_user_four(),
        _ => println!("Selecione um usuario valido!"),
    }




}

fn run_user_one() {
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
    webbrowser::open("https://github.com/httpie/desktop/releases/download/v2024.1.2/HTTPie-Setup-2024.1.2.exe").expect("erro ao baixar httpie no usuario 1");
    webbrowser::open("https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe").expect("erro ao baixar gh cli no usuario 1");

    let os = std::env::consts::OS;
    if os == "windows" {
        run_command("explorer", ["."].to_vec()).expect("Error no OS");
    } else if os == "linux" {
        run_command("nautilus", ["."].to_vec()).expect("Error no OS");
    } else if os == "macos" {
        println!("Mac OS não é suportado ainda")
    }
}

fn run_user_two() {
    run_command("git", ["config", "--global", "user.email", "nicaciosarah085@gmail.com"].to_vec()).expect("Error ao rodar run command no usuario 2");
    run_command("git", ["config", "--global", "user.name", "Sarah Nicacio"].to_vec()).expect("Error ao rodar run command no usuario 2");
    run_command("git", ["config", "--global", "init.defaultBranch", "main"].to_vec()).expect("Error ao rodar run command no usuario 2");
    webbrowser::open("https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe").expect("erro ao baixar gh cli no usuario 2");

}

fn run_user_three() {
    run_command("git", ["config", "--global", "user.email", "vieiravarandagrazielly@gmail.com"].to_vec()).expect("Error ao rodar run command no usuario 3");
    run_command("git", ["config", "--global", "user.name", "Grazielly Varanda"].to_vec()).expect("Error ao rodar run command no usuario 3");
    run_command("git", ["config", "--global", "init.defaultBranch", "main"].to_vec()).expect("Error ao rodar run command no usuario 3");
    webbrowser::open("https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe").expect("erro ao baixar gh cli no usuario 3");
    webbrowser::open("https://github.com/httpie/desktop/releases/download/v2024.1.2/HTTPie-Setup-2024.1.2.exe").expect("erro ao baixar httpie no usuario 4");
    webbrowser::open("https://github.com/cli/cli/releases/download/v2.58.0/gh_2.58.0_windows_amd64.msi").expect("erro ao baixar gh cli no usuario 4");
}

fn run_user_four() {
    run_command("git", ["config", "--global", "user.email", "rrafa7124@gmail.com"].to_vec()).expect("Error ao rodar run command no usuario 4");
    run_command("git", ["config", "--global", "user.name", "Rafael Raposo"].to_vec()).expect("Error ao rodar run command no usuario 4");
    run_command("git", ["config", "--global", "init.defaultBranch", "main"].to_vec()).expect("Error ao rodar run command no usuario 4");
    webbrowser::open("https://github.com/httpie/desktop/releases/download/v2024.1.2/HTTPie-Setup-2024.1.2.exe").expect("erro ao baixar httpie no usuario 4");
    webbrowser::open("https://github.com/cli/cli/releases/download/v2.58.0/gh_2.58.0_windows_amd64.msi").expect("erro ao baixar gh cli no usuario 4");
    webbrowser::open("https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe").expect("erro ao baixar gh cli no usuario 4");
}
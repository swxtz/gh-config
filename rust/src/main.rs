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
            "dev.gustavomendona@protonmail.com",
        ]
        .to_vec(),
    )
    .expect("Error");

    download_font().expect("Erro ao baixar fonte");
}

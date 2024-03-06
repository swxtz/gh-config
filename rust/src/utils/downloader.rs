use crate::utils::checksum::check256;
use crate::utils::unzip::unzip;
use std::fs::File;

/// Download the fonts compressed in a .zip from the jetbrains website
pub fn download_font() -> Result<(), Box<dyn std::error::Error>> {
    let font_url = "https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip";
    let font_checksum = "6f6376c6ed2960ea8a963cd7387ec9d76e3f629125bc33d1fdcd7eb7012f7bbf";
    let zip_location = "./jetbrains_mono.zip";
    let unzip_location = "jetbrains_mono";

    let mut response = reqwest::blocking::get(font_url).expect("Erro no request");

    if response.status().is_success() {
        let mut local_file = File::create(zip_location)?;
        response.copy_to(&mut local_file)?;

        let verify_checksum = check256(font_checksum, zip_location);

        println!("CHECKSUM.............. {}", verify_checksum);

        println!("Arquivo ZIP baixado com sucesso para: {}", zip_location);

        unzip(zip_location);

        println!("Arquivo descompactado para: {}", unzip_location);
    } else {
        println!("Erro: {}", response.status());
    }

    Ok(())
}

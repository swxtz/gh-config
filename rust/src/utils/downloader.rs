use std::fs::{copy, File, OpenOptions};
use std::io::Cursor;
use std::path::Path;
use reqwest::blocking::get;


pub fn download_font() -> Result<(), Box<dyn std::error::Error>> {
    let font_url = "https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip";
    let zip_location = "./jetbrains_mono.zip";
    let unzip_location = "jetbrains_mono";

    let mut response = reqwest::blocking::get(font_url).expect("Erro no request");

    if response.status().is_success() {
        let mut local_file = File::create(zip_location)?;
        response.copy_to(&mut local_file)?;

        println!("Arquivo ZIP baixado com sucesso para: {}", zip_location);

        //unzip_file(zip_location, unzip_location);

        println!("Arquivo descompactado para: {}", unzip_location);
    } else {
        println!("Erro: {}", response.status());
    }

   Ok(())
}

use std::{fs::{File, create_dir_all}, io::Write};

use crate::logger::custom::{custom_message, Colors};


pub fn create_file()  {
    let filepath = "./config.yaml";

    create_dir_all("./config").expect("Não foi possível criar o diretório de configuração");

    let mut file = File::create(filepath).expect("Não foi possível criar o arquivo de configuração");
    let content = "";
    file.write_all(content.as_bytes()).expect("Não foi possível escrever no arquivo de configuração");

    custom_message("Arquivo de configuração criado com sucesso".to_string(), Colors::Green)
}
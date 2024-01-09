use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
   pub os: String,
}

pub fn read_file() -> Config {
    let file = fs::File::open("config.yaml").expect("Não foi possível abrir o arquivo de configuração");
    let scrape_config: Config = serde_yaml::from_reader(file).expect("Não foi possível ler o arquivo de configuração");

    print!("{:?}", scrape_config.os);
    return scrape_config;
}
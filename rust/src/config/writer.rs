use std::fs;

use crate::config::schema::ConfigFile;

use super::schema::Os;

// create a serializer yaml using serde_yaml
pub fn write_os(value: Os) {
    let config = ConfigFile {
        os: value,
    };

    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("{}", yaml);
    fs::write("config.yaml", yaml).expect("Unable to write file");
    
}

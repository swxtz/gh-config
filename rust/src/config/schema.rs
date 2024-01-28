use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Os {
    Linux,
    Macos,
    Windows,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub os: Os,
    pub schema: String,
}

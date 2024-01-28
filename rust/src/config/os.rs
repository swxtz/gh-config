use crate::logger::custom::{custom_message, Colors};
use std::thread;
use std::time::Duration;

use super::schema;
use super::writer::write_os;

pub fn verify_os() {
    #[warn(unused_assignments)]
    custom_message(
        "Verificando sistema operacional...".to_string(),
        Colors::Yellow,
    );

    let sleep_duration = Duration::from_millis(500);
    thread::sleep(sleep_duration);

    if cfg!(target_os = "linux") {
        let os = "linux";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
        write_os(schema::Os::Linux);
    } else if cfg!(target_os = "macos") {
        let os = "macos";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
        write_os(schema::Os::Macos);
    } else if cfg!(target_os = "windows") {
        let os = "windows";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
        write_os(schema::Os::Windows);
    } else {
        let os = "unknown";
        custom_message(format!("Sistema operacional: {}", os), Colors::Red);
        write_os(schema::Os::Unknown);
    }
}

use crate::logger::custom::{custom_message, Colors, self};
use std::time::Duration;
use std::thread;

pub fn verify_os() {
    #[warn(unused_assignments)]
    let mut os = "unknown";

    custom_message("Verificando sistema operacional...".to_string(), Colors::Yellow);
    
    let sleep_duration = Duration::from_millis(500);
    thread::sleep(sleep_duration);

    if cfg!(target_os = "linux") {
        os = "linux";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
    } else if cfg!(target_os = "macos") {
        os = "macos";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
    } else if cfg!(target_os = "windows") {
        os = "windows";
        custom_message(format!("Sistema operacional: {}", os), Colors::Green);
    } else {
        os = "unknown";
        custom_message(format!("Sistema operacional: {}", os), Colors::Red);
    }

    

}
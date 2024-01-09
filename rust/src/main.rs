
mod config;
mod logger;

use crate::config::{create_file::create_file, os::verify_os};
fn main() {
    create_file();
    verify_os();
   
    println!("Hello, world!");
}

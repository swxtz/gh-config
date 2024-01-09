
mod config;
mod logger;

use crate::config::{os::verify_os};
fn main() {
    //create_file();
    verify_os();
   
    println!("Hello, world!");
}

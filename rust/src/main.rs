
mod config;
mod logger;

use crate::config::{os::verify_os};
fn main() {
    verify_os();
   
    println!("Hello, world!");
}

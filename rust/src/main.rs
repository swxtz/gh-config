
mod config;
mod logger;

use config::create_file;

use crate::config::create_file::create_file;
fn main() {
    create_file();
   
    println!("Hello, world!");
}

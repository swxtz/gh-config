 use colored::Colorize;

pub enum Colors {
    Green,
}

pub fn custom_message(message: String, color: Colors ) {
    match color {
        Colors::Green => println!("{}", message.green()),
    }
}
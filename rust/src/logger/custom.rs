 use colored::Colorize;

pub enum Colors {
    Green,
    Yellow,
    Red,
}

pub fn custom_message(message: String, color: Colors ) {
    match color {
        Colors::Green => println!("{}", message.green()),
        Colors::Yellow => println!("{}", message.yellow()),
        Colors::Red => println!("{}", message.red()),
    }
}
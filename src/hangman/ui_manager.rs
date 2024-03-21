use std::io::{self, Write};

pub fn print_header() -> () {
    println!("***************************************");
    println!("*** Welcome to the Hangman Program! ***");
    println!("***************************************");
}

pub fn print_intoduction(config: &Config) -> () {
    println!("Hello, {}!", config.get_user_name());
}

pub fn print_menu() -> () {
    println!("1. Play a game");
    println!("2. Change word file name");
    println!("3. Change user name");
    println!("4. Exit");
    print!("Select an option: ")
    io::stdout().flush(); // print! requires flush to display for this use case
}
use std::io::{self, Write};
use std::fs::File;

use crate::{game_setup, hangman, word_manager};


fn main() {
    println!("***************************************");
    println!("*** Welcome to the Hangman Program! ***");
    println!("***************************************");

    let config = match File::open(".config") {
        Err(_) => game_setup::first_time_setup(),
        Ok(file_handle) => game_setup::setup(file_handle),
    };

    // create hangman game instance
//    let hangman_instance = match hangman::initialize(&config) {
//        Ok(h) => h,
//        Err(err_msg) => {
//            println!("Error: {}", err_msg);
//            return;
//        }
//    };

    let hangman_instance = Hangman::new("Hello World!");
    let cached_words = Vec::<String>::new();

    println!("Hello, {}!", config.get_user_name());
    
    loop {
        // let user select choice
        let mut input_string = String::new();

        if let Err(err_msg) = io::stdin().read_line(&mut input_string) {
            panic(":: Unable to read from command line: {err_msg} ::");
        }

        let chosen_option: u8 = match input_string.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        }

        let choice = match determine_menu_option(chosen_option) {
            Ok(choice) => choice,
            Err(msg) => println!(":: {} ::", msg),
        };

        match choice {
            "PLAY" => todo!(),
            "EDIT_FILE_NAME" => todo!(),
            "CHANGE_NAME" => todo!(),
            "EXIT" => break;
        }
    }
    
    println!("Thank you for playing Hangman!");
}


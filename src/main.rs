use std::io::{self, Write};
use std::fs::File;

use hangman::{game_setup, hangman::Hangman};

fn main() {
    println!("***************************************");
    println!("*** Welcome to the Hangman Program! ***");
    println!("***************************************");

    let config = match File::open(".config") {
        Err(_) => game_setup::first_time_setup(),
        Ok(file_handle) => game_setup::setup(file_handle),
    };

    // create hangman game instance
    let mut hangman_instance = match hangman::initialize(&config) {
        Ok(h) => h,
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    };

    // let mut hangman_instance = Hangman::new("Hello World!".to_string());
    let cached_words = Vec::<String>::new();

    println!("Hello, {}!", config.get_user_name());
    
    loop {
        hangman::print_menu_lines();

        // let user select choice
        let mut input_string = String::new();
        
        if let Err(err_msg) = io::stdin().read_line(&mut input_string) {
            panic!(":: Unable to read from command line: {err_msg} ::");
        }

        let chosen_option: usize = match input_string.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let choice = match hangman::determine_menu_option(chosen_option) {
            Ok(ret_choice) => ret_choice,
            Err(msg) => {
                println!(":: {} ::", msg);
                continue;
            },
        };

        println!("{}", choice);

        match choice {
            "PLAY" => {
                let result = hangman::hangman::play_game(&mut hangman_instance);
                println!("{}", match result {
                    Ok(_) => "You win! Play again?",
                    Err(_) => "You lost! Try again?",
                });
            },
            "EDIT_FILE_NAME" => {
                
            },
            "CHANGE_NAME" => {
            
            },
            "EXIT" => break,
            other => {
                panic!("Unknown choice returne from determine_menu_option: {}", other);
            }
        }
    }
    
    println!("Thank you for playing Hangman!");
}


use std::io;
use std::fs::File;

use hangman_rs;
use hangman_rs::game_setup;
use hangman_rs::hangman::Hangman;


fn main() {
    println!("***************************************");
    println!("*** Welcome to the Hangman Program! ***");
    println!("***************************************");

    let config = match File::open(".config") {
        Err(_) => game_setup::first_time_setup(),
        Ok(file_handle) => game_setup::setup_with_handle(file_handle),
    };

    let mut words = Vec::<String>::new();

    if let Err(_) = hangman_rs::load_words(&mut words, &config) {
        println!("Error loading words from file");
        return;
    }

    println!("Hello, {}!", config.get_user_name());
    
    loop {
        hangman_rs::print_menu_lines();

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

        let choice = match hangman_rs::determine_menu_option(chosen_option) {
            Ok(ret_choice) => ret_choice,
            Err(msg) => {
                println!(":: {} ::", msg);
                continue;
            },
        };

        println!("{}", choice);

        match choice {
            "PLAY" => {
                let word = hangman_rs::get_random_word(&words);
                let mut hangman_instance = Hangman::new_with_icon(word, config.get_life_icon());
                hangman_instance.play_game();
            },
            "EDIT_FILE_NAME" => {
                
            },
            "CHANGE_NAME" => {
            
            },
            "EXIT" => break,
            other => {
                panic!("Unknown choice returned from determine_menu_option: {}", other);
            }
        }
    }
    
    println!("Thank you for playing Hangman!");
}


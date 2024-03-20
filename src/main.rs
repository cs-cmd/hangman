mod hangman;

use std::io::{self, Write};
//use crate::hangman::word_manager::WordManager;
use crate::hangman::word_manager;

fn main() {
    let ast_line = "***************************************"; 
    println!("{ast_line}");
    println!("*** Welcome to the Hangman Program! ***");
    println!("{ast_line}");
    
    // try to read .config file
    // if file doesn't exist, create it after getting username and filename

    let mut user_name = String::new();
    let mut file_name = String::new();

    loop {
        print!("Please enter your name: ");
        let _ = io::stdout().flush();
        
        if let Ok(_) = io::stdin().read_line(&mut user_name) {
            break;
        }        
        
        println!("Invalid name value, please re-enter.");
    }

    let hangman_instance = loop {
        print!("Please enter file that contains words for game: ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut file_name);

        match hangman::initialize(&file_name) {
            Ok(instance) => break instance,
            Err(NotFound) => println!("File not found"),
            Err(err_msg) => println!("Error: {:?}", err_msg)
        }
    };

    println!("The selected word is: {}", hangman_instance);
}


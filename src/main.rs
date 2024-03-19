mod hangman;

use std::io;
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
    
        if let Ok(_) = io::stdin().read_line(&mut user_name) {
            break;
        }        
        
        println!("Invalid name value, please re-enter.");
    }

    loop {
        print!("Please enter file that contains words for game: ");

        if let Ok(_) = io::stdin().read_line(&mut file_name) {
            break;
        }

        println!("Filename could no be read in; please try again.");
    }

    println!("Hangman starting...");
    // let word_manager = match WordManager::new(&file_name) {
    //     Err(err_msg) => panic!("WordManager could not be created; cause: {}", err_msg),
    //     Ok(wm) => wm,
    // };
    // let selected_word = word_manager::choose_random_word(&file_name);
    match word_manager::choose_random_word(&file_name) {
        Err(_) => println!("Cannot choose word"),
        Ok(word) => println!("Selected word: {}", &word),
    }
}


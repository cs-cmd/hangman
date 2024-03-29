use rand::Rng;
use std::{io::{self, Write}, fs};

pub mod hangman;
pub mod game_setup;

use crate::game_setup::Config;

static MENU_CHOICES: [(&str, &str); 4] = [
    ("1. Play a game", "PLAY"),
    ("2. Change word file name", "EDIT_FILE_NAME"),
    ("3. Change user name", "CHANGE_NAME"),
    ("4. Exit", "EXIT"),
];

pub fn determine_menu_option<'a>(choice: usize) -> Result<&'a str, String> {
    if choice > MENU_CHOICES.len() {
        return Err("Invalid choice".to_string());
    }

    return Ok((MENU_CHOICES[choice-1]).1);
}

pub fn print_menu_lines() {
    // print menu
    for line in MENU_CHOICES {
        println!("{}", line.0);
    }

    print!("Select an option: ");

    // print! doesn't flush by default
    let _ = io::stdout().flush();
}

pub fn load_words(cached_words: &mut Vec<String>, config: &Config) -> Result<(), String> {
    let file_name = config.get_file_name();

    let words = match fs::read_to_string(file_name) {
        Err(_) => return Err("Unable to read word file".to_string()),
        Ok(words) => words,
    };

    if words.len() == 0 {
        return Err("No words in file".to_string());
    }

    for line in words.lines() {
        cached_words.push(line.trim().to_string());
    }

    return Ok(());
}

pub fn get_random_word(cached_words: &Vec<String>) -> String {
    // select random number    
    let rand_num: usize = rand::thread_rng().gen::<usize>() % (cached_words.len() - 1);

    return cached_words[rand_num].clone();
}

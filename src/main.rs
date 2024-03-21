mod hangman;

use std::io::{self, Write};
//use crate::hangman::word_manager::WordManager;
use crate::hangman::{word_manager, ui_manager, game_setup};

fn main() {
    ui_manager::print_header();

    let config = match File::open(".config") {
        Err(_) => game_setup::first_time_setup(),
        Ok(file_handle) => game_setup::setup(file_handle),
    }

    ui_manager::print_introduction();
    
    let mut chosen_option: u8 = loop {
        ui_manager::print_menu();

        let mut input_string = String::new();

        if let Err(err_msg) = io::stdin().read_line(&mut input_string) {
            panic!("Unable to read from command line: {err_msg}");
        }

        if let Ok(num) = input_string.trim().parse::<u8>() {
            break num;
        }

        println!("Please enter a number");
    }

    // handle menu options
}


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

    println!("The selected word is: {}", hangman_instance);
}


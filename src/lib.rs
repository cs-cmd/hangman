use rand::Rng;
use std::fs;

pub mod hangman;
pub mod game_setup;
pub mod ui_manager;
pub mod word_manager;

static menu_choices: [(&str, &str)] = [
    ("1. Play a game", "PLAY"),
    ("2. Change word file name", "EDIT_FILE_NAME"),
    ("3. Change user name", "CHANGE_NAME"),
    ("4. Exit", "EXIT"),
];

pub fn determine_menu_option(choice: u8) -> Result<&'a str, <'a str> {
    let max_choice = menu_choices.len();
    if chosen_option < 0 || chosen_option >= max_choice {
        return Err("Please enter a number between 1 and {}", max_choice);
    }

    return Ok(menu_choices[choice].1);
}

pub fn print_menu_lines() {
    // print menu
    for line in menu_choices {
        println!("{}", line.0);
    }

    print!("Select an option: ");

    // print! doesn't flush by default
    let _ = io::stdout().flush();
}

// Continuously reads from files. Can be overly large. Consider making struct that stores words that can be used
// without needing to re-read file
pub fn load_words<'a>(&mut cached_words: Vec<String>, config: &Config) -> Result<(), &'a str> {
    let file_name = config.get_file_name();

    let words = match fs::read_to_string(file_name) {
        Err(_) => return Err("Unable to read word file"),
        Ok(words) = > words,
    };

    if words.len() == 0 {
        return Err("No words in file");
    }

    for line in words.lines() {
        cached_words.push(line.trim().to_string());
    }

    return Ok(());
}

pub fn get_random_word<'a>(chached_words: Vec<String>) -> String {
    // select random number    
    let rand_num: usize = rand::thread_rng().gen::<usize>() % (cached_words.len() - 1);

    return String::from(&cached_words[rand_num]);
}

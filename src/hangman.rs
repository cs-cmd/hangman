pub mod word_manager;
pub mod ui_manager;

use std::collections::HashSet;

// can also use HashMap to combine the two `guesses` maps into a single
// variables <char, bool> (if guesses, is either true or false if it contains
// the character
pub struct Hangman {
    chosen_word: String,
    successful_guesses: HashSet::<char>,
    wrong_guesses: HashSet::<char>,
    lives: u8,
}

const LIVES_MAX: u8 = 5;
const LIFE_ICON: char = 'x';

impl Hangman {
    pub fn new(word: &str) -> Hangman {
        return Hangman {
            chosen_word: String::from(word),
            successful_guesses: HashSet::new(),
            wrong_guesses: HashSet::new(),
            lives: LIVES_MAX,
        };
    }

    // PRINTS [ x x x x x ];
    pub fn print_lifebar(&self) -> String {
        let mut lifebar = String::from("[");

        for life in 1..=LIVES_MAX {
            lifebar.push_str({
                if life < self.lives {
                    // format!(" {}", LIFE_ICON); // returns String
                    // concat!(" {}", LIFE_ICON); // this expects string literal 
                    " x" // Write code to allow for custom user life point icons
                } else {
                    " _"
                }
            });
        }

        lifebar.push_str(" ]");

        return lifebar;
    }

    pub fn generate_word_hint(&self) -> String {
        let mut word_hint = String::new();

        for char_at in self.chosen_word.chars() {
            if !char_at.is_alphabetic() {
                word_hint.push(char_at);
                continue;
            }

            word_hint.push(match self.successful_guesses.get(&char_at) {
                Some(x) => *x,
                None => '_'
            });
        }

        return word_hint;
    }
}

#[allow(dead_code)]
pub fn print_header() {
    todo!();
}

pub fn initialize(file_name: &str) -> Hangman {
    // init word manager
    // get word from manager
    // load word into hangman instance, 
    // return to 

    let chosen_word = match word_manager::choose_random_word(file_name) {
        Ok(word) => word,
        Err(err_msg) => panic!("Error: {:?}", err_msg),
    };
    
    return Hangman::new(&chosen_word);
}
pub mod word_manager;
pub mod ui_manager;
pub mod game_setup;

use std::collections::HashSet;
use std::fmt;

// can also use HashMap to combine the two `guesses` maps into a single
// variables <char, bool> (if guesses, is either true or false if it contains
// the character
pub struct Hangman {
    chosen_word: String,
    successful_guesses: HashSet::<char>,
    wrong_guesses: HashSet::<char>,
    lives: u8,
    life_icon: char,
}

impl Hangman {
    const MAX_LIVES: u8 = 5;
    const DEFAULT_LIFE_ICON: char = 'x';

    pub fn new(word: String) -> Hangman {
        return Hangman {
            chosen_word: word,
            successful_guesses: HashSet::new(),
            wrong_guesses: HashSet::new(),
            lives: MAX_LIVES,
            life_icon: DEFAULT_LIFE_ICON,
        };
    }
    pub fn new_with_options(word: String, life_icon: char) -> Hangman {
        return Hangman {
            chosen_word: word,
            successful_guesses: HashSet::new(),
            wrong_guesses: HashSet::new(),
            lives: MAX_LIVES,
            life_icon,
        };
    }

    // PRINTS [ x x x x x ];
    pub fn print_lifebar(&self) -> String {
        let mut lifebar = String::from("[");

        for life in 1..=MAX_LIVES {
            lifebar.push(' ');

            lifebar.push(if life <= self.lives {
                self.life_icon
            } else {
                ' '
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

impl fmt::Display for Hangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Hangman instance word: {}", self.chosen_word);
    }
}

pub fn initialize(config: &Config) -> Result<Hangman, &'static str> {
    let chosen_word = word_manager::choose_random_word(config.get_file_name())?;

    return Ok(Hangman::new_with_options(chosen_word, config.get_life_icon()));
}

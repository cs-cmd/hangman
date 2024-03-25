// HANGMAN
// Represents the game state
use std::collections::HashSet;
use std::fmt;
use crate::game_setup::Config;

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

    pub fn new(word: String) -> Hangman {
        return Hangman {
            chosen_word: word,
            successful_guesses: HashSet::new(),
            wrong_guesses: HashSet::new(),
            lives: Self::MAX_LIVES,
            life_icon: Config::get_default_life_icon(),
        };
    }

    pub fn new_with_options(word: String, life_icon: char) -> Hangman {
        return Hangman {
            chosen_word: word,
            successful_guesses: HashSet::new(),
            wrong_guesses: HashSet::new(),
            lives: Self::MAX_LIVES,
            life_icon,
        };
    }

    // PRINTS [ x x x x x ];
    pub fn create_lifebar(&self) -> String {
        let mut lifebar = String::from("[");

        for life in 1..=Self::MAX_LIVES {
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

    pub fn create_word_hint(&self) -> String {
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

    pub fn guess_letter<'a>(&self, guessed_char: char) -> Result<&'a str, &'a str> {
        if let Some(_) = self::successful_guesses.get(guessed_char) {
            return Err("Already successfully guessed '{}'", guessed_char);
        }

        if let Some(_) = self::wrong_guesses.get(guessed_char) {
            return Err("Already incorrectly guessed '{}'", guessed_char);
        }

        for char_at in self::chosen_word.chars() {
            if guessed_char == char_at {
                successful_guesses.insert(guessed_char);
                return Ok("{} exists in the word!", guessed_char);
            }
        }

        self::lives -= 1;
        wrong_guesses.insert(guessed_char);
        return Ok("{} is not in the word.", guessed_char);
    } 

    pub fn get_config(&self) -> &Config -> {
        return self::config;
    }
}

impl fmt::Display for Hangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Hangman instance word: {}", self.chosen_word);
    }
}

pub fn initialize(config: &Config) -> Result<(Hangman, Vec<String>), &'static str> {
    todo!();
}

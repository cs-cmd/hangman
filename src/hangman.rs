// HANGMAN
// Represents the game state
use std::collections::HashSet;
use std::{fmt, io::{self, Write}};
use crate::game_setup::{self, Config};

// can also use HashMap to combine the two `guesses` maps into a single
// variables <char, bool> (if guesses, is either true or false if it contains
// the character
pub struct Hangman {
    chosen_word: String,
    unique_letters_left: u8
    successful_guesses: HashSet::<char>,
    wrong_guesses: HashSet::<char>,
    lives: u8,
    life_icon: char,
}

enum GuessResult {
    CorrectGuess,
    IncorrectGuess,
    NoLivesLeft,
    Won,
    AlreadyGuessed,
    Continue
}

impl Hangman {
    const MAX_LIVES: u8 = 5;

    pub fn new(word: String) -> Hangman {
        let unique_letters = HashSet::new();

        for char in word.chars() {
            if char
        }
        

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

    pub fn guess_letter(&mut self, guessed_char: char) -> GuessResult {
        let char_upper = guessed_char.to_ascii_uppercase();

        if let Some(_) = self.successful_guesses.get(&char_upper) {
            return GuessResult::AlreadyGuessed;
        } 

        if let Some(_) = self.wrong_guesses.get(&char_upper) {
            return GuessResult::AlreadyGuessed;
        }

        for char_at in self.chosen_word.chars() {
            if char_upper == char_at.to_ascii_uppercase() {
                self.unique_letters_left -= 1;
                // store letters as uppercase when successfully guessed
                self.successful_guesses.insert(char_upper);
                return GuessResult::CorrectGuess;
            }
        }

        self.lives -= 1;

        if self.lives == 0 {
            return GuessResult::NoLivesLeft;
        }

        self.wrong_guesses.insert(char_upper);
        return GuessResult::IncorrectGuess;
    } 

    pub(self) fn guess_word(&mut self, guessed_word: String) -> GuessResult {
        todo!();
    }
}

impl fmt::Display for Hangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Hangman instance word: {}", self.chosen_word);
    }
}

pub fn initialize(config: &Config) -> Result<(Hangman, Vec<String>), String> {
    
}

pub fn play_game(hangman: &mut Hangman) -> Result<(), ()> {
    loop {
        println!("{}", hangman.create_lifebar());
        println!("{}", hangman.create_word_hint());
        print!("Select a letter: ");
        let _ = io::stdout().flush();

        let mut input_string: String = String::new();
        if let Err(err) = io::stdin().read_line(&mut input_string) {
            panic!("Unable to read from terminal: {}", err);
        }

        let trimmed_str = input_string.trim();

        let result = match trimmed_str.len() {
            0 => {
                println!("No input entered");
                GuessResult::Continue
            },
            1 => hangman.guess_letter(trimmed_str.chars().next().expect("Expected single char")),
            len => hangman.guess_word(trimmed_str.to_string()),       
        };

        match result {
            GuessResult::Won => return Ok(()),
            GuessResult::NoLivesLeft => return Err(()),
            _ => continue,
        };
    }
}

// HANGMAN
// Represents the game state
use std::collections::{HashSet, HashMap};
use std::{fmt, io::{self, Write}};
use crate::game_setup::Config;

pub struct Hangman {
    word: String,
    unique_letters: HashSet::<char>,
    unique_letters_left: u8,
    guesses: HashMap<char, bool>,
    lives: u8,
    life_icon: char
}

enum GuessResult {
    Correct,
    Incorrect,
    NoLivesLeft,
    Victory,
    AlreadyGuessed
}

impl Hangman {
    const MAX_LIVES: u8 = 6;

    const BODY_PARTS: [char; 6] = [
        '/', // left leg
        '\\', // right leg
        '/', // right arm
        '\\', // left arm
        '|', // torso
        'O', // head
    ];

    pub fn new(word: String) -> Hangman {
        return Self::new_with_icon(word, Config::get_default_life_icon());
    }

    pub fn new_with_icon(word: String, icon: char) -> Hangman {
        // Store word as uppercase
        let word_upper: String = word.to_ascii_uppercase();
        let mut unique_letters_left: u8 = 0;
        let mut unique_letters = HashSet::<char>::new();

        for char_at in word_upper.chars() {
            if !char_at.is_alphabetic() {
                continue;
            }

            if let None = unique_letters.get(&char_at) {
                unique_letters_left += 1;
                unique_letters.insert(char_at);
            }
        }

        return Hangman {
            word: word_upper,
            unique_letters_left,
            unique_letters,
            guesses: HashMap::<char, bool>::new(),
            lives: Self::MAX_LIVES,
            life_icon: icon,
        };
    }

    fn create_life_bar(&self) -> String {
        let mut life_bar = String::from('[');

        for life in 1..=Self::MAX_LIVES {
            life_bar.push(' ');

            life_bar.push({
                if life <= self.lives {
                    self.life_icon
                } else {
                    '_'
                }
            });
        }

        life_bar.push_str(" ]");

        return life_bar;
    }

    fn create_word_hint(&self) -> String {
        let mut hint_string = String::new();

        for char_at in self.word.chars() {
            if !char_at.is_alphabetic() {
                hint_string.push(char_at);
                continue;
            }

            hint_string.push({
                if let Some(true) = self.guesses.get(&char_at) {
                    char_at
                } else {
                    '_'
                }
            });
        } 

        return hint_string;
    }

    fn draw_hangman_ascii(&self) -> () {
        let mut count: u8 = 0;

        let print_parts = Self::BODY_PARTS
            .map(|part| {
                count += 1;
                if count > self.lives {
                    part
                } else {
                    ' '
                }
            }
        );

        // print hangman ascii with values obtained from `map`
        println!("_______  ");
        println!("|     |  ");
        println!("|     {}  ", print_parts[5]);
        println!("|    {}{}{} ", print_parts[0], print_parts[4], print_parts[1]);
        println!("|    {} {} ", print_parts[2], print_parts[3]);
        println!("|        ");
        println!("|-------|");
    }

    fn guess_letter(&mut self, guessed_char: char) -> GuessResult {
        let char_upper = guessed_char.to_ascii_uppercase();

        if let Some(_) = self.guesses.get(&char_upper) {
            return GuessResult::AlreadyGuessed;
        }

        let was_correct = match self.unique_letters.get(&char_upper) {
            Some(_) => {
                self.unique_letters_left -= 1;
                true
            },
            None => {
                self.lives -= 1;
                false
            },
        };

        self.guesses.insert(char_upper, was_correct);

        return {
            if was_correct && self.unique_letters_left == 0 {
                GuessResult::Victory
            } else if was_correct {
                GuessResult::Correct
            } else if self.lives == 0 {
                GuessResult::NoLivesLeft
            } else {
                GuessResult::Incorrect
            }
        };
    }

    fn guess_word(&mut self, guessed_word: String) -> GuessResult {
        todo!();
    }

    pub fn play_game(&mut self) -> () {
        let end_message = loop {
            println!("{}", self.create_life_bar());
            println!("{}", self.create_word_hint());
            print!("Guess a letter: ");
            let _ = io::stdout().flush();

            let mut input_string = String::new();

            if let Err(_) = io::stdin().read_line(&mut input_string) {
                println!(":: Unable to read from command line ::");
                return;
            }

            // remove new_line char from string
            let trimmed_string = input_string.trim();

            let guess_result = match trimmed_string.len() {
                0 => {
                    println!("Please guess a letter or word");
                    continue;
                },
                1 => self.guess_letter(trimmed_string.chars().next().unwrap()),
                _ => todo!(), 
            };

            match guess_result {
                GuessResult::Victory => break "Victory",
                GuessResult::NoLivesLeft => break "Lost",
                _ => continue,
            };
        };

        println!("");

        println!("{}! The word was {}", end_message, self.word);
    }
}

impl fmt::Display for Hangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Hangman instance word: {}", self.word);
    }
}

pub fn initialize(config: &Config) -> Result<(Hangman, Vec<String>), String> {
    let mut words = Vec::<String>::new();

    if let Err(msg) = crate::load_words(&mut words, config) {
        return Err(format!("Unable to load words from file: {}", msg));
    }

    let first_random_word = crate::get_random_word(&words);

    return Ok((Hangman::new_with_icon(first_random_word, config.get_life_icon()), words));
}

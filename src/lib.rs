pub mod hangman;
pub mod game_setup;
pub mod ui_manager;
pub mod word_manager;

pub enum MenuChoice {
    Valid(u8),
    Invalid
}

pub fn make_menu_choice(input: u8) -> MenuChoice {
    use MenuChoice as MC;
    if input < 0 && input > 4 {
        return MC::Invalid;
    }

    return MC::Valid(input);
}
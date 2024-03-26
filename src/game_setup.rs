// GAME_SETUP
// Handles and is used to represent setup logic for the game
// from filename to aspects of user experience
use std::io::{self, Write, Read};
use std::fs::File;

pub struct Config {
    user_name: String,
    file_name: String,
    life_icon: char,
}

impl Config {  
    pub const DEFAULT_LIFE_ICON: char = 'x';

    pub fn new() -> Config {
        return Self::new_with_options(String::new(), String::new(), Self::DEFAULT_LIFE_ICON);
    }
    
    pub fn new_with_options(user_name: String, file_name: String, life_icon: char) -> Config {
        return Config {
            user_name: user_name,
            file_name: file_name,
            life_icon,
        };
    }

    pub fn write_config_and_return<'a>(user_name: String, file_name: String, life_icon: char) -> Result<Config, String>{
        let mut file_handle = match File::create(".config") {
            Ok(file) => file,
            Err(err_msg) => return Err(format!(":: File could not be opened for writing: {} ::", err_msg)),
        };


        if let Err(err_msg) = file_handle.write(format!("user_name={user_name}\nfile_name={file_name}\nlife_icon={life_icon}").as_bytes()) {
            return Err(format!(":: File could not be written to: {} ::", err_msg));
        };

        return Ok(Self::new_with_options(user_name, file_name, life_icon));
    }

    pub fn get_life_icon(&self) -> char {
        return self.life_icon;
    }
    pub fn get_user_name(&self) -> &str {
        return &self.user_name;
    }
    pub fn get_file_name(&self) -> &str {
        return &self.file_name;
    }
    pub fn get_default_life_icon() -> char {
        return Self::DEFAULT_LIFE_ICON;
    }
}

// NOTE: Creating a string, trimming it (returns a &str), and creating another
// String from that is overkill for a reading a single line from input. 
// TODO: Find resolution that's cleaner and has less memory overhead
pub fn first_time_setup() -> Config {
    let user_name = loop {
        print!("Please enter your name: ");
        let _ = io::stdout().flush();
        
        let mut input_string = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input_string) {
            break input_string.trim().to_string();
        }        
        
        println!(":: Invalid name value, please re-enter ::");
    };

    let file_name = loop {
        print!("Please enter file that contains words for game: ");
        let _ = io::stdout().flush();

        let mut input_string = String::new();
        let _ = io::stdin().read_line(&mut input_string);

        // opening file to test presence; if failure, file not valid
        let _ = match File::open(input_string.trim()) {
            Err(_) => println!("Invalid file name"),
            Ok(_) => break input_string.trim().to_string(),
        };
    };

    let life_icon = loop {
        print!("Enter life icon: ");
        let _ = io::stdout().flush();
        
        let mut icon_string: String = String::new();
        if let Err(_) = io::stdin().read_line(&mut icon_string) {
            println!(":: Error - unable to read from console ::");
            continue;
        }

        if let Some(c) = icon_string.chars().next() {
            break c;
        }

        println!(":: Invalid character ::");
    };

    match Config::write_config_and_return(user_name, file_name, life_icon) {
        Ok(c) => return c,
        Err(_) => return Config::new(),
    };
}

// No serde, manual 
pub fn setup(mut file_handle: File) -> Config {
    let mut file_contents = String::new();

    if let Err(_) = file_handle.read_to_string(&mut file_contents) {
        println!(":: Could not read content from file ::");
        return Config::new();
    }

    const ARR_COPY_VAL: String = String::new();
    let mut config_params: [String; 3] = [ARR_COPY_VAL; 3];

    let mut param_found: [bool; 3] = [false; 3];

    for arg in file_contents.lines() {
        // find index of equal sign, split string into value and 
        let pair = match arg.find('=') {
            Some(index) => (&arg[..index], &arg[index+1..]),
            None => {
                println!(":: Parameter line {} is not valid ::", arg);
                continue;
            }
        };

        // get index of value to add, or continue
        let ind: usize = match pair.0 {
            "user_name" => 0,
            "file_name" => 1,
            "life_icon" => 2,
            other => {
                println!(":: Config parameter {} is not supported: {} ::", other, arg);
                continue;
            },
        };

        // if the parameter was already found, don't use again
        if param_found[ind] {
            println!(":: Parameter already added, skipping ::");
            continue;
        }

        param_found[ind] = true;
        config_params[ind] = pair.1.to_string();        
    }

    // validate all three parameters are found
    // if one of the parameters wansn't found or the length is less than 1, 
    // return false and use default configuration
    let mut all_found: bool = true;
    
    for i in 0..3 {
        if !param_found[i] || config_params[i].len() < 1 {
            all_found = false;
            break;
        }
    }
    
    // use default configuration if not found
    if !all_found {
        println!(":: Config file not valid, using default values ::");
        return Config::new();
    }

    // assign config params to values to pass in
    // if char life_icon is not valid, use the default
    let mut iter = config_params.into_iter();

    let user_name = iter.next().expect("Required username"); 
    let file_name: String = iter.next().expect("Required filename");
    let life_icon: char = match iter.next().expect("Required life icon").chars().next() {
        Some(ch) => ch,
        None => Config::get_default_life_icon(),
    };

    return Config::new_with_options(user_name, file_name, life_icon);
}

// GAME_SETUP
// Handles and is used to represent setup logic for the game
// from filename to aspects of user experience
use std::io::{self, Write, Read};
use std::fs::File;

#[derive(Debug)]
pub struct Config {
    user_name: String,
    file_name: String,
    life_icon: char,
}

impl Config {  
    const DEFAULT_LIFE_ICON: char = 'x';

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

    pub fn write_to_file(&self) -> Result<(), String>{
        let mut file_handle = match File::create(".config") {
            Ok(file) => file,
            Err(_) => return Err("Unable to open config file for writing".to_string()),
        };

        if let Err(_) = file_handle.write(format!("user_name={}\nfile_name={}\nlife_icon={}", 
                                          self.user_name,
                                          self.file_name,
                                          self.life_icon).as_bytes()) {
            return Err("Unable to write config data to file".to_string());
        }

        return Ok(());
    }

    pub fn serialize(c: Config) -> Result<(), String> {
        todo!();
    }

    pub fn deserialize(file_name: &str) -> Result<Config, String> {
        todo!();
    }

    pub fn get_file_name(&self) -> &str {
        return &self.file_name;
    }
    
    pub fn get_user_name(&self) -> &str {
        return &self.user_name;
    }

    pub fn get_life_icon(&self) -> char {
        return self.life_icon;
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

    let config = Config::new_with_options(user_name, file_name, life_icon);

    if let Err(msg) = config.write_to_file() {
        println!(":: Unable to write config to file: {}  ::", msg);
    }

    return config;
}

pub fn setup(file_name: &str) -> Config {
    return match File::open(file_name) {
        Ok(file_handle) => setup_with_handle(file_handle),
        Err(_) => {
            println!(":: Cannot open config file, using default config ::");
            Config::new()
        },
    };
}

// No serde, manual 
pub fn setup_with_handle(mut file_handle: File) -> Config {
    let mut file_contents = String::new();
    if let Err(_) = file_handle.read_to_string(&mut file_contents) {
        println!("Cannot read config file");
        return Config::new();
    }

    let mut param_found: [bool; 3] = [false; 3];
    let mut config_params: Vec<String> = vec![String::new(); 3];
    let mut params_needed: usize = 3;

    for arg in file_contents.lines() {
        // get left and right hand side of equal sign in string
        let pair = match arg.find('=') {
            Some(index) => (&arg[..index], &arg[index + 1..]),
            None => {
                println!(":: Parameter line {} is not valid (NO_EQUALS) ::", arg);
                continue;
            }
        };

        let ind: usize = match pair.0 {
            "user_name" => 0,
            "file_name" => 1,
            "life_icon" => 2,
            _ => {
                println!(":: Parameter {} not used in this program ::", pair.0);
                continue;
            },
        };

        if param_found[ind] {
            println!(":: Parameter already loaded; skipping ::");
            continue;
        }

        param_found[ind] = true;
        // trim param to remove newline char
        config_params[ind] = pair.1.trim().to_string();
        params_needed -= 1;
    }

    // use default configuration if not found
    if params_needed != 0 {
        println!(":: Config file incomplete, using default values ::");
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

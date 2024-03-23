use std::io::{self, Write, Read};
use std::fs::File;

pub struct Config {
    user_name: String,
    file_name: String,
    life_icon: char,
}

impl Config {  
    pub fn new() -> Config {
        return Self::new_with_options(String::new(), String::new(), 'x');
    }
    
    pub fn new_with_options(user_name: String, file_name: String, life_icon: char) -> Config {
        return Config {
            user_name: user_name,
            file_name: file_name,
            life_icon,
        };
    }

    pub fn write_config_and_return<'a>(user_name: String, file_name: String, life_icon: char) -> Result<Config, &'a str>{
        let mut file_handle = match File::create(".config") {
            Ok(file) => file,
            Err(err_msg) => return Err("File could not be opened for writing."),
        };


        if let Err(err_msg) = file_handle.write(format!("user_name={user_name}\nfile_name={file_name}\nlife_icon={life_icon}").as_bytes()) {
            return Err("File could not be written to");
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
    
    let config = match file_handle.read_to_string(&mut file_contents) {
        Ok(_) => {
            let mut user_name = String::new();
            let mut file_name = String::new();
            let mut life_icon: char = 'x';

            // username, filename, lifeicon
            for arg in file_contents.lines() {
                let pair: (&str, &str) = match arg.find('=') {
                    Some(index) => (&arg[..index], &arg[index+1..]), 
                    None => ("NO_EQUAL", "INVALID"),
                };

                match pair.0 {
                    "user_name" => user_name = String::from(pair.1),
                    "file_name" => file_name = String::from(pair.1),
                    "life_icon" => life_icon = match pair.1.chars().next() {
                        // 6 indents deep; not a good look :(
                        Some(ch) => ch,
                        None => {
                            println!("Not a valid life icon character");
                            'x'
                        }
                    },
                    other => println!("Config parameter {} is not supported: {}", arg, other),
                };
            }

            Config::new_with_options(user_name, file_name, life_icon)
        },
        Err(err_msg) => {
            println!("Could not read contents from file");
            Config::new()  
        },
    };

    return config;
}

//pub fn setup_with_serde() -> Config {
//    return todo()!;
//}

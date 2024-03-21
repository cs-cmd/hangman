pub struct Config {
    user_name: String,
    file_name: String
    life_icon: char,
};

impl Config {  
    pub fn write_config(user_name: &str, file_name: &str, life_icon: char) -> (){

    }

    pub fn write_config_and_return(user_name: &str, file_name: &str, life_icon) -> Config {
        write_config(user_name, file_name, life_icon);

        return Config {
            user_name,
            file_name, 
            life_icon,
        };
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

pub fn first_time_setup() -> Config {
    let mut user_name: String;
    let mut file_name: String;
    let mut life_icon: char;

    loop {
        print!("Please enter your name: ");
        let _ = io::stdout().flush();
        
        if let Ok(_) = io::stdin().read_line(&mut user_name) {
            break;
        }        
        
        println!("Invalid name value, please re-enter.");
    }

    loop {
        print!("Please enter file that contains words for game: ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut file_name);

        // opening file to test presence; if failure, file not valid
        let _ = match File::open(&file_name) {
            Err(_) => println!("Invalid file name"),
            Ok(_) => break;
        }
    };

    loop {
        print!("Enter life icon: ");
        let _ = io::stdout().flush();
        
        let mut icon_string: String;
        if let Err(_) = io::stdin().read_line(&mut icon_string) {
            println!("Invalid life icon, please enter a single character.");
            continue;
        }

        if let Ok(c) = icon_string.chars().next() {
            life_icon = c;
            break;
        }

        println!("Invalid character.");
    }

    return write_config_and_return(&user_name, &file_name, life_icon);
}
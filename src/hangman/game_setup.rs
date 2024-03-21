pub struct Config {
    user_name: String,
    file_name: String
    life_icon: char,
};

impl Config {  
    pub fn new() -> Config {
        return Config {
            user_name: String::new(),
            file_name: String::new(),
            life_icon: 'x';
        }
    }
    
    pub fn new_with_options(user_name: &str, file_name: &str, life_icon: char) -> Config {
        return Config {
            user_name: String::from(user_name),
            file_name: String::from(file_name),
            life_icon,
        };
    }

    pub fn write_config(user_name: &str, file_name: &str, life_icon: char) -> Result<(). &'static str>{
        let mut file_handle = File::create(".config")?;
        file_handle.write(format!("user_name={user_name}\nfile_name={file_name}\nlife_icon={life_icon}").as_bytes())?;
        return Ok(());
    }

    pub fn write_config_and_return(user_name: &str, file_name: &str, life_icon) -> Config {
        write_config(user_name, file_name, life_icon);
        if let Err(err_msg) = write_config(user_name, file_name, life_icon) {
            println!(":: Error - could not create config file - {err_msg} ::");
        }
        return Config::new_with_options(user_name, file_name, life_icon);
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
        
        println!(":: Invalid name value, please re-enter ::");
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
            println!(":: Error - unable to read from console ::");
            continue;
        }

        if let Ok(c) = icon_string.chars().next() {
            life_icon = c;
            break;
        }

        println!(":: Invalid character ::");
    }

    return write_config_and_return(&user_name, &file_name, life_icon);
}
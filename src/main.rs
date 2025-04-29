use std::io;
use std::fs::File;
use loadingbar::LoadingBar;
use std::time::Duration;
use serde_json::Value;
use std::io::Read;

mod art;
mod settings;
mod locations;
mod save;

fn main() {
    print!("{}[2J", 27 as char);
    
    println!("  *    P O K E M O N :  R U S T E D   R E D    *");
    println!("");
    println!(" ___  ___      _        ___  ___                   ");
    println!(" |  \\/  |     (_)       |  \\/  |                   ");
    println!(" | .  . | __ _ _ _ __   | .  . | ___ _ __  _   _   ");
    println!(" | |\\/| |/ _` | | '_ \\  | |\\/| |/ _ \\ '_ \\| | | |  ");
    println!(" | |  | | (_| | | | | | | |  | |  __/ | | | |_| |  ");
    println!(" \\_|  |_/\\__,_|_|_| |_| \\_|  |_/\\___|_| |_|\\__,_|  ");
    println!("");
    
    main_action();
}

fn main_action(){
    println!("   0: New game (Previous game will be overwritten)");
    println!("   1: Load Game");
    println!("   2: Options");
    println!("   3: Exit game");
    println!("");
    

    let mut input = String::new();
    let mut choice = 0;
    
        //Take main menu input
        io::stdin()
        .read_line(&mut input)
          .expect("Failed to read line");
      
      if input.trim().parse::<i32>().is_err() {
          invalid_choice(main);
      }
      else {
          choice = input.trim().parse::<i32>().unwrap();
      }
  
      match choice {
          0 => initialize_new_game(),
          1 => save::load_game(),
          2 => println!("Options"),
          3 => exit(),
          _ => invalid_choice(main),
      };
}   

fn invalid_choice(argument: fn()){
    println!("Invalid choice, try again");
    println!("");
    argument();
}

fn exit(){
    println!("Exiting Game...");
    std::process::exit(0);
}

fn initialize_new_game(){

    let bar = LoadingBar::new_with_config(Duration::new(3, 0), '*', 50, String::from("Initializing..."));
    bar.start();
    print!("{}[2J", 27 as char);
    intro();
}

fn intro(){

    art::print_person_art("oak");
    println!("");
    println!("Hello there!");
    println!("Welcome to the world of Pokémon! My name is Oak! People call me the Pokémon Professor!");
    println!("This world is inhabited by creatures called Pokémon! For some people, Pokémon are pets. Other use them for fights. Myself… I study Pokémon as a profession.");
    println!("");
    'name: loop{
        println!("What is your name?");
        println!("");
        let mut name = String::from("");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let choice = name;
        println!();
        println!("Your name is {}? (Y/N)", choice.trim());
        println!();
        'sure: loop{    
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() {
                "y" =>  {settings::edit_json("src/settings/general_settings.json", "name", choice.trim()); break 'name},
                "Y" =>  {settings::edit_json("src/settings/general_settings.json", "name", choice.trim()); break 'name},
                "n"=> break 'sure,
                "N"=> break 'sure,
                _ => println!("Invalid choice, try again")
                
            }
        } 
    }
    println!();

    let name = check_json("src/settings/general_settings.json", "name");
    println!("Right!\nSo your name is {}!", name);

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
    println!("This is my grandson. He's been your rival since you were a baby.");
    art::print_person_art("gary");

    
    'name: loop{
        println!("…Erm, what is his name again?");
        println!("");
        let mut name = String::from("");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let choice = name;
        println!();
        println!("His name is {}? (Y/N)", choice.trim());
        println!();
        'sure: loop{    
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() {
                "y" =>  {settings::edit_json("src/settings/general_settings.json", "rivalname", choice.trim()); break 'name},
                "Y" =>  {settings::edit_json("src/settings/general_settings.json", "rivalname", choice.trim()); break 'name},
                "n"=> break 'sure,
                "N"=> break 'sure,
                _ => println!("Invalid choice, try again")
                
            }
        } 
    }
    
    let name = check_json("src/settings/general_settings.json", "rivalname");
    print!("{}[2J", 27 as char);
    art::print_person_art("gary");
    println!("That's right! I remember now!\n His name is  {}!", name);

    println!("");
    println!("Your very own pokemon journey is about to unfold!");
    println!("A world of dreams and adventures with pokemon awaits!");
    println!("Let's Go!!!");
    
    io::stdin()
        .read_line(&mut String::new())
        .expect("Something went seriously wrong here");

    locations::enter_location("red_house".to_string());
}


/////////// READ A FILE ///////////////
pub fn check_json(path: &str, target: &str) -> String {
    //Open the json file
    // Using expect for slightly better error messages than unwrap()
    let mut file = File::open(path).expect(&format!("Failed to open file: {}", path));
    let mut contents = String::new();
    // Using expect here too - also handle potential read error
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read file: {}", path));

    //Parse the json
    // Using expect here too
    let json_value: Value = serde_json::from_str(&contents)
        .expect(&format!("Failed to parse JSON from file: {}", path));

    //Get the value
    if let Value::Object(map) = json_value {
        if let Some(value) = map.get(target) {
            // Attempt to get the value as a string slice (&str)
            if let Some(str_val) = value.as_str() {
                // If successful, convert the &str to an owned String and return it
                return str_val.to_string();
            } else {
                // The key exists, but the value is not a string
                eprintln!(
                    "Warning: Value for key '{}' in '{}' is not a string. Found: {:?}",
                    target, path, value
                );
                // Return an empty string or handle as an error condition
                return String::new(); // Or perhaps panic, or return Result
            }
        } else {
            // The key 'target' was not found in the JSON object
            eprintln!("Warning: Key '{}' not found in '{}'.", target, path);
            return String::new(); // Or perhaps panic, or return Result
        }
    } else {
        // The top-level JSON structure was not an object
        eprintln!("Warning: JSON file '{}' does not contain a top-level object.", path);
        return String::new(); // Or perhaps panic, or return Result
    }


}


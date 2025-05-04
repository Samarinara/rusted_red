use crate::locations;
use crate::settings;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn save_game(location: &String){
    println!("Saving game...");
    settings::edit_json("src/save/save.json", "previous_location", &location);
}

pub fn set_current_location(location: String){
    settings::edit_json("src/save/save.json", "current_location", &location);
}

pub fn load_game(){
    println!("Loading game... :)");

    let previous_location = crate::check_json("src/save/save.json", "previous_location");
    locations::enter_location(previous_location);
}

pub fn check_gate(name: &String) -> bool{
    //Load the locations file
    let mut file = File::open("src/save/gates.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");
         
    // Set a variable to the contents
    let save_file: HashMap<String, bool> = serde_json::from_str(&contents).unwrap();
    println!("deserialized");

    if let Some(gate) = save_file.get(name) {
        if gate == &true{
            println!("{} is true", gate);
            return true;
        }else{
            println!("{} is false", gate);
            return false;
        }

    }

    return false; 
}

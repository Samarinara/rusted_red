use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde::Deserialize;
use std::io;
use crate::save;

#[derive(Deserialize, Debug)]
struct Location{
    name: String,
    desc: String,
    art: String,
    category: String,
    options: HashMap<i32, HashMap<String, String>>
}

pub fn enter_location(name: String){

    //Load the locations file
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");
    
    // Set a variable to the contents
    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();
    println!("deserialized");
    
    if let Some(location) = locations.get(&name) {
        print!("{}[2J", 27 as char);
        println!("{}", location.art);
        println!("{}", location.desc);

        println!("");
        println!("What would you like to do?");
        
        let mut sorted_options: Vec<(&i32, &HashMap<String, String>)> = location.options.iter().collect();
        sorted_options.sort_by_key(|(key, _)| *key);
        
        for (key, value) in sorted_options {
            println!("{}: {}", key, value.get("text").unwrap());
        }
        
       println!("");

        save::set_current_location(name.clone());

        loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        
            // The match function changes depending on location.options
            match input.trim().parse::<i32>(){
                Ok(choice_key) =>{
                    if let Some(option) = location.options.get(&choice_key){
                        if location.category == "location" {
                            println!("You chose location: {}", option.get("text").unwrap());
                            save::save_game(name);
                            println!("{}", option.get("next").unwrap());
                            enter_location(option.get("next").unwrap().to_string());
                            break;
                        }else if location.category == "interaction" {
                            println!("You chose interaction: {}", option.get("next").unwrap());
                            save::save_game(name);

                            break;
                        }

                    }else{
                        println!("Invalid choice, try again");
                    }
                }

                Err(_) => {println!("Invalid choice, try again");}
                
            }
        }

    }else{
        println!("Location {} not found", name);
    }


}

pub fn load_interaction(name: String){
    println!("Loading interaction: {}", name);
    
    //Load the locations file
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");

    // Set a variable to the contents
    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();
    println!("deserialized");

    if let Some(location) = locations.get(&name) {
        print!("{}[2J", 27 as char);
        println!("{}", location.art);
        println!("{}", location.desc);
    }else{
        println!("Interaction {} not found", name);
    }

}

pub fn find_location(target_name: &str) -> String {
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();

    for (key, data) in &locations {
        if data.name == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}    

pub fn find_interaction(target_name: &str) -> String {
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();

    //

    for (key, data) in &locations {
        if data.name == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}
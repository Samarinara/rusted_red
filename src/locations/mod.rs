use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde::Deserialize;
use std::io;
use crate::art;
use crate::save;
use crate::func;

#[derive(Deserialize, Debug)]
struct Location{
    name: String,
    desc: String,
    art: String,
    options: HashMap<i32, HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct LocationFunction{
    name: String,
    desc: String,
    art: String,
    options: HashMap<i32, HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct Conversation{
    art: String,
    lines: HashMap<String, String>,
    next: String

}

#[derive(Deserialize, Debug)]
struct GatedLocation{
    gate: String,
    if_true: String,
    if_false: String
}


pub fn enter_location(name: String){

    println!("{}", determine_category(&name));

    //Load the locations file
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");
    
    // Set a variable to the contents
    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();
    println!("deserialized");
    
    if let Some(location) = locations.get(&name) {
      //  print!("{}[2J", 27 as char);
        println!("\n\n\n\n\n\n");
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
                        println!("about to load {}", &option.get("next").unwrap().to_string());
                        load_unknown(&option.get("next").unwrap().to_string());
                    }
                    else{
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

pub fn load_location_function(name: String){

    //Load the locations file
    let mut file = File::open("src/locations/location_functions.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");
    
    // Set a variable to the contents
    let locations: HashMap<String, LocationFunction> = serde_json::from_str(&contents).unwrap();
    println!("deserialized");
    
    if let Some(location) = locations.get(&name) {
      //  print!("{}[2J", 27 as char);
        println!("\n\n\n\n\n\n");
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
                        println!("about to load {}", &option.get("next").unwrap().to_string());
                        //Call the function
                        func::call_gameplay_fuction_from_string(&option.get("function").unwrap().to_string());
                        //Load the next scene
                        load_unknown(&option.get("next").unwrap().to_string());
                    }
                    else{
                        println!("Invalid choice, try again");
                    }
                }

                Err(_) => {println!("Invalid choice, try again");}
                
            }
        }

    }else{
        println!("Location function {} not found", name);
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

pub fn load_conversation(name: String) {
        //Load the conversations file
        let mut file = File::open("src/locations/conversations.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);    
        println!("loaded");
        
        // Set a variable to the contents
        let conversations: HashMap<String, Conversation> = serde_json::from_str(&contents).unwrap();
        println!("deserialized");
        
        if let Some(convo) = conversations.get(&name) {
         //   print!("{}[2J", 27 as char);
            println!("\n\n\n\n\n\n");
            art::print_person_art(&convo.art);

            let mut index = 0;
            loop{
                println!("{}", convo.lines.get(&index.to_string()).unwrap());
               
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
               
                index += 1;

                if convo.lines.get(&index.to_string()).is_none() {
                    println!("end of convo");
                    load_unknown(&convo.next);
                    break;
                }
            }

        }
}

pub fn check_gated_location(name: String){
    let mut file = File::open("src/locations/gated_locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let gated_locations: HashMap<String, GatedLocation> = serde_json::from_str(&contents).unwrap();
 
    if let Some(gated_convo) = gated_locations.get(&name) {
        if save::check_gate(&gated_convo.gate){

            load_unknown(&gated_convo.if_true);

        }else{

            load_unknown(&gated_convo.if_false);

        }
    
    }
}

pub fn load_route(name: String){
    println!("{}", name);
}


fn load_unknown(name: &String){

    println!("{}", name);

    let _name = name.trim().to_string();

    let category = determine_category(&name);

    if category == "location"{
        println!("loading location {}", &_name);
        enter_location(_name);

    }
    else if category == "interaction"{
        println!("loading interaction {}", &_name);
        load_interaction(_name);

    }
    else if category == "conversation"{
        println!("loading conversation {}", &_name);
        load_conversation(_name);

    }
    else if category ==  "gated_location"{
        println!("loading gated location {}", &_name);
        check_gated_location(_name);

    }else if category == "location_function"{
        println!("loading location function {}", &_name);
        load_location_function(_name);
    }
    else if category == "route" {
        println!("loading route {}", &_name);
        load_route(_name);
    }
    else{
        println!("nowhere to go");
    }
    
}

fn determine_category(name: &String) -> String {

    if find_location(&name) != "nothing found" {
        return "location".to_string();
    }

    if find_interaction(&name) != "nothing found"{
        return "interaction".to_string();
    }

    if find_conversation(&name) != "nothing found"{
        return "conversation".to_string();
    }

    if find_gated_location(&name) != "nothing found"{
        return "gated_location".to_string();
    }

    if find_location_function(&name) != "nothing found"{
        return "location_function".to_string();
    }

    return "there is bullshit here".to_string();
}


pub fn find_location(target_name: &str) -> String {
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();

    for (key, _data) in &locations {
        if key == target_name {
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

pub fn find_conversation(target_name: &str) -> String {
    let mut file = File::open("src/locations/conversations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let conversations: HashMap<String, Conversation> = serde_json::from_str(&contents).unwrap();

    for (key, _data) in &conversations {
        if key == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}

pub fn find_gated_location(target_name: &str) -> String {
    let mut file = File::open("src/locations/gated_locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let gated_locations: HashMap<String, GatedLocation> = serde_json::from_str(&contents).unwrap();

    for (key, _data) in &gated_locations {
        if key == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}

pub fn find_location_function(target_name: &str) -> String {
    let mut file = File::open("src/locations/location_functions.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    

    let location_functions: HashMap<String, LocationFunction> = serde_json::from_str(&contents).unwrap();

    for (key, _data) in &location_functions {
        if key == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}






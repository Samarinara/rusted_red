use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde::Deserialize;
use std::io;
use crate::save;
use crate::func;

#[derive(Deserialize, Debug)]
struct Location{
    name: String,
    desc: String,
    art: String,
    category: String,
    options: HashMap<i32, HashMap<String, String>>,
    function: Option<String>
}

#[derive(Deserialize, Debug)]
struct Conversation{
    art: String,
    lines: HashMap<String, String>,
    next: String

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
        //Load the locations file
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
            println!("{}", convo.art);

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
                    break;
                }
            }

            load_unknown(&convo.next);


        }
}


fn load_unknown(name: &String){
    let _location = "location";
    let _interaction = "interaction";
    let _conversation = "conversation";

    let _name = name.to_string();

    let category = determine_category(&name);

    if category == _location{
        println!("loading location {}", &_name);
        enter_location(_name);
    }
    else if category == _interaction{
        println!("loading interaction {}", &_name);
        load_interaction(_name);
    }
    else if category == _conversation{
        println!("loading conversation {}", &_name);
        load_conversation(_name);
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

    return "there is bullshit here".to_string();
}


pub fn find_location(target_name: &str) -> String {
    let mut file = File::open("src/locations/locations.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    
    let locations: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();
    
    let error = "nothing found";

    if let Some(location) = locations.get(target_name) {    
        for (key, data) in &locations {
            if data.name == target_name {
                return key.to_string();
            }
        }
    }else{
        return error.to_string();
    }
    
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

    let locations: HashMap<String, Conversation> = serde_json::from_str(&contents).unwrap();

    for (key, _data) in &locations {
        if key == target_name {
            return key.to_string();
        }
    }

    let error = "nothing found";
    return error.to_string();
}
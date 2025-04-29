use crate::locations;
use crate::settings;

pub fn save_game(location: String){
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

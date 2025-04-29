use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde_json::Error as SerdeError;


//////////// LOAD A FILE ///////////////
fn load_json(path: &str) -> Result<HashMap<String, String>, SerdeError> {
    // Open the file
    let file = File::open(path)
    .expect("Failed to open json");
let reader = BufReader::new(file);

// Deserialize JSON into a HashMap<String, String>
serde_json::from_reader(reader)
}

/////////// DRAW A POKEMON //////////////
pub fn print_pokemon_art(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let map = load_json("src/art/pokemonascii.json")?;

    if let Some(sprite) = map.get(name) {
        println!("{}", sprite);
        Ok(())
    } else {
        eprintln!("No ASCII art found for Pokémon '{}'", name);
        Err(format!("No art for '{}'", name).into())
    }
}

////////// DRAW A PERSON ///////////////
pub fn print_person_art(name: &str) -> Result<(), Box<dyn std::error::Error>> {
let map = load_json("src/art/peopleart.json")?;

if let Some(sprite) = map.get(name) {
    // println! preserves all newlines exactly
    println!("{}", sprite);
    Ok(())
} else {
    eprintln!("No ASCII art found for person '{}'", name);
    Err(format!("No art for '{}'", name).into())
}
}


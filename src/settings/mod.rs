use std::fs::File;
use std::io::Write;
use std::io::Read;
use serde_json::Value;
use std::path::Path;
use std::error::Error;

///////// CHANGE A SETTING ////////////
pub fn edit_json<P: AsRef<Path>>(
    file_path: P,
    name: &str,
    value: &str,
    ) -> Result<(), Box<dyn Error>> {
    // 1. Read the JSON file content into a string.
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 2. Parse the JSON string into a mutable serde_json::Value.
    // We use `mut` because we intend to modify it.
    let mut json_value: Value = serde_json::from_str(&contents)?;

    // 3. Access the JSON object and modify the value by name.
    // We expect the top level of our settings file to be a JSON object.
    if let Value::Object(ref mut map) = json_value {
        // Use `insert` to add or update the key-value pair.
        // insert returns the old value if the key existed, or None if it didn't.
        map.insert(name.to_string(), Value::String(value.to_string()));
        println!("Setting '{}' updated to '{}'", name, value);
    } else {
        // If the top level is not an object, we cannot change a setting by name.
        return Err(format!("JSON file does not represent a top-level object, cannot change setting '{}'", name).into());
    }

    // 4. Serialize the modified serde_json::Value back to a JSON string.
    // We use `to_string_pretty` for nice formatting in the output file.
    let modified_contents = serde_json::to_string_pretty(&json_value)?;

    // 5. Write the modified JSON content back to the original file.
    // We truncate the file and write the new content.
    let mut file = File::create(&file_path)?;
    file.write_all(modified_contents.as_bytes())?;

Ok(())
}

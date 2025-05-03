pub fn call_gameplay_fuction_from_string(func: &str){
    match func{
        "heal_party" => heal_party(),
        _ => println!("Invalid function")
    };

}

fn heal_party(){
    println!("Healing party...");
}



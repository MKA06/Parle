use serde::Deserialize;

use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::error::Error;

// struct to represent the data.json file
#[derive(Debug, Deserialize)]
pub struct Data {
    pub commands: HashMap<String, String>,
    pub phrases: HashMap<String, String>,
}

// read the contents of the JSON, return it as the Data struct
pub fn read_data_from_file() -> Result<Data, Box<dyn Error + Send + Sync>> {
    let file = File::open("data.json").expect("nooo");
    let reader = BufReader::new(file);

    let data : Data = serde_json::from_reader(reader)?;

    Ok(data)
}

// return the appropriate response
pub fn return_response(data: &Data, message : &str)  -> String {
    let message = message.to_lowercase();

    let response = String::new();
    
    // if it ends with !, it must be a valid command. 
    // if it doesn't, check for phrases. 
    if message.ends_with("!") {
        if let Some(response) = data.commands.get(&message) {
            return response.clone();
        }
    } else {
        let mut response = String::new();
        for (key, value) in &data.phrases {
            if message.contains(key) {
                response.push_str(value);
                response.push(' ');
            }
        }
        return response.trim().to_string();
    }
    response
}
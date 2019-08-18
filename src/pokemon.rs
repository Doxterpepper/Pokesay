use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub pokemon: String,
    pub number: i32,
    pub say: String,
}

pub fn read_pokemon(file: &str) -> Vec<Pokemon> {
    let br = BufReader::new(File::open(file).unwrap());
    serde_json::from_reader(br).expect("Error parsing json file")
}

pub fn find_pokemon(pokemon: &Vec<Pokemon>, search_string: String) -> Option<&Pokemon> {
    if search_string == "" {
        return None;
    }

    for poke in pokemon {
        if poke.pokemon == search_string {
            return Some(&poke);
        }
    }

    None
}



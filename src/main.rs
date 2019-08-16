extern crate rand;
extern crate clap;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use clap::{App, Arg};
use rand::distributions::{IndependentSample, Range};

#[derive(Serialize, Deserialize)]
struct Pokemon {
    pokemon: String,
    number: i32,
    say: String,
}

fn read_pokemon(file: &str) -> Vec<Pokemon> {
  let br = BufReader::new(File::open(file).unwrap());
  serde_json::from_reader(br).expect("Error parsing json file")
}

fn main() {
  let matches = App::new("pokesay")
    .version("1.0")
    .author("Dock O'Neal <mail@dockoneal.com>")
    .about("like pokemonsay, which is like cowsay, but written in rust")
    .arg(Arg::with_name("resource")
      .short("r")
      .value_name("FILE")
      .help("Set custom pokeomon resource file")
      .takes_value(true))
    .arg(Arg::with_name("message")
      .short("m")
      .value_name("MESSAGE")
      .help("message to include with pokemon")
      .required(false))
    .arg(Arg::with_name("pokemon")
      .short("p")
      .value_name("POKEMON")
      .help("specify which pokemon to display")
      .required(false))
      .get_matches();

  let resource_file = matches.value_of("pokesay")
    .unwrap_or("./pokemon.json");

  let message = matches.value_of("message")
    .unwrap_or("");

  let pokemon = read_pokemon(resource_file);

  let between = Range::new(0, pokemon.len());
  let mut rng = rand::thread_rng();
  let selected_pokemon = &pokemon[between.ind_sample(&mut rng)];
  println!("{}", selected_pokemon.say);
  println!("{}", selected_pokemon.pokemon);
  println!("{}", message);
}

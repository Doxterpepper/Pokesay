extern crate rand;
extern crate clap;

use clap::{App, Arg};
use rand::distributions::{IndependentSample, Range};
mod pokemon;

fn select_random_pokemon(pokemon: &Vec<pokemon::Pokemon>) -> &pokemon::Pokemon {
  let between = Range::new(0, pokemon.len());
  let mut rng = rand::thread_rng();
  &pokemon[between.ind_sample(&mut rng)]
}

fn select_pokemon(pokemon: &Vec<pokemon::Pokemon>, user_input: String) -> &pokemon::Pokemon {
  // find_pokemon will return None if user_input is empty
  if let Some(pokemon) = pokemon::find_pokemon(pokemon, user_input) {
    pokemon
  } else {
    select_random_pokemon(pokemon)
  }
}

fn main() {
  let matches = App::new("pokesay")
    .version("1.0")
    .author("Dock O'Neal <mail@dockoneal.com>")
    .about("like pokemonsay, like cowsay, but built with rust")
    .arg(Arg::with_name("pokedex")
      .short("d")
      .value_name("FILE")
      .help("Specify a pokedex")
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

  // Empty string will default to random pokemon
  let user_pokemon_selection = matches.value_of("pokemon")
    .unwrap_or("");

  let pokemon = pokemon::read_pokemon(resource_file);

  let selected_pokemon = select_pokemon(&pokemon, String::from(user_pokemon_selection));

  println!("{}", selected_pokemon.say);
  println!("{}", selected_pokemon.pokemon);
  println!("{}", message);
}

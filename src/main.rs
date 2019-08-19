extern crate rand;
extern crate clap;

use std::path::Path;
use std::process;
use clap::{App, Arg, ArgMatches};
use rand::distributions::{IndependentSample, Range};
mod pokemon;

const DEFAULT_POKEDEX: &'static str = "/usr/share/pokedex/pokedex.json";

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

// Resolves pokedex file. Will be user provided pokedes, pokedex in the user's home folder
// or a pokedex in /urs/share/pokedex
// Returns a string or prints message to console and exits program.
fn resolve_pokedex(matches: &ArgMatches) -> String {
    if matches.is_present("pokedex") {
        let pokedex_path = matches.value_of("pokedex").unwrap();
        if Path::new(pokedex_path).exists() {
            return String::from(pokedex_path)
        } else {
            println!("Unable to find pokedex at {}", pokedex_path);
            process::exit(1);
        }
    } else {
        let home_pokedex = [env!("HOME"), ".pokedex.json"].join("/");
        if Path::new(&home_pokedex).exists() {
            return home_pokedex
        }
    }

    if !Path::new(DEFAULT_POKEDEX).exists() {
        println!("Unable to find default pokedex at {}", DEFAULT_POKEDEX);
        process::exit(1);
    }
    String::from(DEFAULT_POKEDEX)
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


    let message = matches.value_of("message")
        .unwrap_or("");

    // Empty string will default to random pokemon
    let user_pokemon_selection = matches.value_of("pokemon")
        .unwrap_or("");

    let pokedex = resolve_pokedex(&matches);
    let pokemon = pokemon::read_pokemon(&pokedex);

    let selected_pokemon = select_pokemon(&pokemon, String::from(user_pokemon_selection));

    println!("{}", selected_pokemon.say);
    println!("{}", selected_pokemon.pokemon);
    println!("{}", message);
}

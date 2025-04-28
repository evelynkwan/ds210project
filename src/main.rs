mod data;
mod graph;
mod clustering;
mod analysis;
mod tests;

use crate::data::load_pokemon_data;
use graph::Graph;

fn main() {
    let path = "pokedex.csv"; 
    let pokemons = load_pokemon_data(path).expect("Failed to load Pokemon data");

    println!("Loaded {} Pokémon.", pokemons.len());
}

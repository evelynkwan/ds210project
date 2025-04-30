// runs the full project pipeline
mod data;
mod graph;
mod clustering;
mod analysis;
mod tests;

use data::{load_pokemon_data, normalize_pokemons};
use graph::Graph;
use clustering::find_clusters;
use analysis::print_clusters;


fn main() {
    let path = "pokedex.csv"; 
    let mut pokemons = load_pokemon_data(path).expect("Failed to load Pokemon data");

    println!("Loaded {} Pokémon.", pokemons.len());
    normalize_pokemons(&mut pokemons);

    let k = 5; // each pokemon connects to 5 most similar
    let pokemon_graph = Graph::build_knn_graph(&pokemons, k);

    let clusters = find_clusters(&pokemon_graph);

    print_clusters(&pokemons, &clusters);
}

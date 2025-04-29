// analyzes and prints clusters
use crate::data::Pokemon;
use std::collections::HashSet;

pub fn print_clusters(pokemons: &[Pokemon], clusters: &[HashSet<usize>]) {
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}: {} Pokémon", i + 1, cluster.len());

        // prints first 5 pokemon names
        for &index in cluster.iter().take(5) {
            println!("- {}", pokemons[index].name);
        }
        println!("---");
    }
}

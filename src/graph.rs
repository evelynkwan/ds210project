// builds a k-NN graph of pokemon based on stat similarity
use crate::data::Pokemon;
use std::collections::HashMap;

// represents the pokemon graph
pub struct Graph {
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency: HashMap::new(),
        }
    }

    // computes euclidean distance between two pokemon
    fn distance(a: &Pokemon, b: &Pokemon) -> f64 {
        a.base_stats()
            .iter()
            .zip(b.base_stats().iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    // builds a k-nearest neighbor graph
    pub fn build_knn_graph(pokemons: &[Pokemon], k: usize) -> Self {
        let mut graph = Graph::new();

        for (i, p1) in pokemons.iter().enumerate() {
            // calculate distances to all other pokemon
            let mut distances: Vec<(usize, f64)> = pokemons
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, p2)| (j, Graph::distance(p1, p2)))
                .collect();

            // sort by distance and select k nearest neighbors
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let neighbors = distances.iter().take(k).map(|&(j, _)| j).collect();

            graph.adjacency.insert(i, neighbors);
        }
        graph
    }
}

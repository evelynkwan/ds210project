// clusters the graph by finding connected components
use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

pub fn find_clusters(graph: &Graph) -> Vec<HashSet<usize>> {
    let mut visited = HashSet::new();
    let mut clusters = Vec::new();

    for &node in graph.adjacency.keys() {
        if visited.contains(&node) {
            continue;
        }

        let mut cluster = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while let Some(curr) = queue.pop_front() {
            if visited.insert(curr) {
                cluster.insert(curr);
                if let Some(neighbors) = graph.adjacency.get(&curr) {
                    for &neighbor in neighbors {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        clusters.push(cluster);
    }

    clusters
}

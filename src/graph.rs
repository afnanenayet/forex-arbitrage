//! Module for defining the graph structure as well as convenience methods for the graph

use std::collections::HashMap;

/// A graph structure that contains a representation of edges and weights. The topmost keys
/// represent vertices, and each vertex has a HashMap of associated weights.
pub type Graph<K, V> = HashMap<K, HashMap<K, V>>;

/// Convenient type alias
pub type ForexGraph = Graph<String, f32>;

/// An implementation of the Bellman-Ford algorithm for cycle detection in a directed graph
///
/// The cycle detection algorithm needs to know which node to start from, when looking for a cycle,
/// which is specified by `starting_node`
pub fn detect_cycle<K>(graph: &Graph<K, f32>, starting_node: &K) -> bool
where
    K: std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
{
    // used to keep track of the distance from the starting node
    let mut distances: HashMap<&K, f32> = HashMap::new();
    distances.insert(starting_node, 0.0);

    // allows us to backtrack and reconstruct the path from the last node
    let mut predecessor: HashMap<&K, &K> = HashMap::new();

    // repeatedly relax the distances from the source node
    for _ in 0..graph.keys().len() {
        for u in graph.keys() {
            for v in graph[u].keys() {
                let weight = graph[u][v];
                let u_dist = *distances.get(u).unwrap_or(&std::f32::INFINITY);
                let v_dist = *distances.get(v).unwrap_or(&std::f32::INFINITY);

                if u_dist + weight < v_dist {
                    distances.insert(v, distances[u] + graph[u][v]);
                    predecessor.insert(v, u);
                }
            }
        }
    }

    // check for a negative weight cycle
    for u in graph.keys() {
        for v in graph[u].keys() {
            if distances[u] + graph[u][v] < distances[v] {
                return true;
            }
        }
    }
    false
}

/// Given a graph, detect whether there is a cycle, using each currency as the starting node
pub fn detect_any_cycle<K>(graph: &Graph<K, f32>) -> HashMap<&K, bool>
where
    K: std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
{
    let mut res = HashMap::new();

    for key in graph.keys() {
        let cycle = detect_cycle(graph, key);
        res.insert(key, cycle);
    }
    res
}

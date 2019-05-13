//! Module for defining the graph structure as well as convenience methods for the graph

use std::collections::HashMap;

/// Type alias for the type that represents a conversion rate.
pub type Rate = f64;

/// A graph structure that contains a representation of edges and weights. The topmost keys
/// represent vertices, and each vertex has a `HashMap` of associated weights.
pub type Graph<K, V> = HashMap<K, HashMap<K, V>>;

/// Convenient type alias
pub type ForexGraph = Graph<String, Rate>;

/// An implementation of the Bellman-Ford algorithm for cycle detection in a directed graph
///
/// The cycle detection algorithm needs to know which node to start from, when looking for a cycle,
/// which is specified by `starting_node`. If a cycle is detected, then the cyclic path will be
/// returned. Otherwise, this function returns `None`.
pub fn detect_cycle<'a, K>(graph: &'a Graph<K, Rate>, starting_node: &'a K) -> Option<Vec<&'a K>>
where
    K: std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
{
    // used to keep track of the distance from the starting node
    let mut distances: HashMap<&K, Rate> = HashMap::new();
    distances.insert(starting_node, 0.0);

    // allows us to backtrack and reconstruct the path from the last node
    let mut predecessor: HashMap<&K, &K> = HashMap::new();

    // repeatedly relax the distances from the source node
    for _ in 0..graph.keys().len() {
        for u in graph.keys() {
            for v in graph[u].keys() {
                let weight = graph[u][v];
                let u_dist = *distances.get(u).unwrap_or(&std::f64::INFINITY);
                let v_dist = *distances.get(v).unwrap_or(&std::f64::INFINITY);

                if u_dist + weight < v_dist {
                    distances.insert(v, u_dist + weight);
                    predecessor.insert(v, u);
                }
            }
        }
    }

    // check for a negative weight cycle
    for u in graph.keys() {
        for v in graph[u].keys() {
            // If there is a negative cycle, then calculate the path it takes to get that negative
            // cycle and return it. We can figure out the path using backtracking and the
            // predecessor table.
            if distances[u] + graph[u][v] < distances[v] {
                let mut path = Vec::new();
                let mut current_vert = u;

                // This condition detects the cycle, otherwise reconstructing the path is an
                // infinite loop
                while !(path.contains(&current_vert) || current_vert == starting_node) {
                    path.insert(0, &current_vert);
                    current_vert = predecessor[current_vert]
                }
                path.insert(0, &current_vert);

                // TODO(afnan) clean this up
                // Right now, we need to prune
                let mut i = 1;

                while i < path.len() {
                    if path[i] == path[0] {
                        break;
                    }
                    i += 1;
                }

                if i < path.len() {
                    return Some(path[0..=i].to_vec());
                }
            }
        }
    }
    None
}

/// Given a graph, detect whether there is a cycle, using each currency as the starting node
pub fn detect_any_cycle<K>(graph: &Graph<K, Rate>) -> HashMap<&K, Option<Vec<&K>>>
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

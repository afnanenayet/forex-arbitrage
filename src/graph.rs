//! Module for defining the graph structure as well as convenience methods for the graph

use std::collections::HashMap;

/// A graph structure that contains a representation of edges and weights. The topmost keys
/// represent vertices, and each vertex has a HashMap of associated weights.
pub type Graph<K, V> = HashMap<K, HashMap<K, V>>;

/// Convenient type alias
pub type ForexGraph = Graph<String, f32>;

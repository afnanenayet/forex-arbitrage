//! This module handles acquiring data from external sources and parsing them in a format that is
//! easy for the rest of the library to understand.

use crate::graph::ForexGraph;
use failure::Error;
use reqwest;
use reqwest::Client;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::iter::FromIterator;

/// Given some currency, this method constructs the weighted edge for the graph with respect to
/// the given currency.
///
/// `currency` is the currency code that the API will recognize. If this is incorrect, then the
/// request will fail.
fn get_currency_data(client: &mut Client, currency: &str) -> Result<HashMap<String, f32>, Error> {
    let url = format!("https://api.exchangeratesapi.io/latest?base={}", currency);

    // Extract the part of the JSON response that yields edge weights
    let map: Value = client.get(&url).send()?.json()?;
    let rate_map: HashMap<String, f32> = serde_json::from_value(map["rates"].clone())?;
    Ok(rate_map)
}

/// Construct a graph that can be used for arbitrage, starting from some base currency code. This
/// method will try to chase every path from the currencies that the base currency can convert to.
/// For example, if "USD" can convert to "EUR" and "CAD", this method will acquire the data for
/// both of those currencies, until the graph is complete.
pub fn construct_graph(client: &mut Client, base_currency: &str) -> Result<ForexGraph, Error> {
    // Create a queue of currencies to handle, with the base currency as the first thing on the
    // queue
    let mut graph = ForexGraph::new();
    let mut queue: Vec<&str> = Vec::new();
    queue.push(base_currency);

    while queue.len() > 0 {
        let currency = queue.pop().unwrap();
        let edges: HashMap<String, f32> = get_currency_data(client, &currency)?;
        let mut cloned_edges: HashMap<String, f32> = HashMap::new();

        for key in edges.keys() {
            if !graph.contains_key(key) {
                queue.push(key);
            }
            cloned_edges.insert(key.clone(), edges[key].clone());
        }
        graph.insert(String::from(currency), cloned_edges);
    }
    Ok(graph)
}

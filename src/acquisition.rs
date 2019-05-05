//! This module handles acquiring data from external sources and parsing them in a format that is
//! easy for the rest of the library to understand.

use crate::graph::ForexGraph;
use failure::Error;
use futures::{Future, Stream};
use reqwest;
use reqwest::Client;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::{fs, time};

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

    // transform the values with the negative log so it's compatible with the bellman-ford search
    let transformed = HashMap::from_iter(rate_map.iter().map(|(k, v)| (k.clone(), -1.0 * v.ln())));
    Ok(transformed)
}

/// Asynchronously query the FOREX API data
fn fetch(url: &str) -> impl Future<Item = reqwest::r#async::Response, Error = reqwest::Error> {
    reqwest::r#async::Client::new().get(url).send()
}

/// Construct a graph that can be used for arbitrage, starting from some base currency code. This
/// method will try to chase every path from the currencies that the base currency can convert to.
/// For example, if "USD" can convert to "EUR" and "CAD", this method will acquire the data for
/// both of those currencies, until the graph is complete.
pub fn construct_graph(client: &mut Client, base_currency: &str) -> Result<ForexGraph, Error> {
    // Create a queue of currencies to handle, with the base currency as the first thing on the
    // queue
    let mut graph = ForexGraph::new();
    let mut queue: Vec<String> = Vec::new();
    queue.push(base_currency.to_string());

    while !queue.is_empty() {
        let currency = queue.pop().unwrap();
        let edges: HashMap<String, f32> = get_currency_data(client, &currency)?;
        let mut cloned_edges: HashMap<String, f32> = HashMap::new();

        for key in edges.keys() {
            if !graph.contains_key(key) {
                queue.push(key.to_string());
            }
            cloned_edges.insert(key.clone(), edges[key]);
        }
        graph.insert(currency, cloned_edges);
    }
    Ok(graph)
}

/// Given some graph, save the graph to disk as a JSON file
/// `file_name` dictates the file name that the disk will be saved to. If no filename is supplied,
/// the default format will be `$UNIX_TIME-forex-graph.json`
pub fn save_graph(graph: &ForexGraph, file_name: Option<String>) -> Result<(), Error> {
    let time = &time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)?
        .as_nanos();
    let file_name = file_name.unwrap_or_else(|| format!("{}-forex-graph.json", &time));

    // Try serializing the graph and saving to disk
    let json_graph = serde_json::to_string(graph)?;
    fs::write(file_name, json_graph)?;
    Ok(())
}

// Given a file name, load a graph from the serialized JSON file
pub fn load_graph(file_name: &str) -> Result<ForexGraph, Error> {
    let json = std::fs::read_to_string(file_name)?;
    let graph: ForexGraph = serde_json::from_str(&json)?;
    Ok(graph)
}

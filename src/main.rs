use acquisition::{construct_graph, save_graph};
use reqwest;

mod acquisition;
mod bellman_ford;
mod graph;

fn main() -> Result<(), Box<std::error::Error>> {
    // Get the text response
    //let resp_text = reqwest::get("https://api.exchangeratesapi.io/latest?base=USD")?.text()?;
    //let map: serde_json::Value = serde_json::from_str(&resp_text)?;
    let mut client = reqwest::Client::new();
    let map = construct_graph(&mut client, "USD")?;
    println!("{:#?}", map);

    // Save map as a json file
    save_graph(&map, None)?;
    Ok(())
}

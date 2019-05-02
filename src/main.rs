use reqwest;
use serde_json;

mod acquisition;
mod bellman_ford;
mod graph;

fn main() -> Result<(), Box<std::error::Error>> {
    // Get the text response
    let resp_text = reqwest::get("https://api.exchangeratesapi.io/latest?base=USD")?.text()?;
    let map: serde_json::Value = serde_json::from_str(&resp_text)?;
    println!("{:#?}", map);
    Ok(())
}

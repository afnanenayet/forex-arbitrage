use acquisition::{construct_graph, save_graph};
use reqwest;
use structopt::StructOpt;

mod acquisition;
mod bellman_ford;
mod graph;

/// Command line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "forex-arbitrage",
    about = "Graph based forex arbitrage detection"
)]
struct Opt {
    /// (optional) The path to a serialized graph file
    #[structopt(short = "i", long = "graph-file")]
    graph_file: Option<String>,

    /// (optional) The file name to save the serialized graph. If no filename is supplied, this
    /// will use the default filename format: "$UNIX_EPOCH-forex-graph.json"
    save_file: Option<String>,
}

impl Opt {
    /// Verify that the opts are in a valid configuration
    pub fn verify(&self) -> bool {
        // Either `graph_file` or `save_file` can be set
        if self.graph_file.is_none() && self.save_file.is_none() {
            return false;
        }

        if self.graph_file.is_some() && self.save_file.is_some() {
            return false;
        }
        true
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();

    if !opt.verify() {
        panic!("Improper arguments");
    }

    if let graph_file = opt.graph_file {
    } else {
        let mut client = reqwest::Client::new();
        let map = construct_graph(&mut client, "USD")?;
        save_graph(&map, None)?;
    }
    // Get the text response
    //let resp_text = reqwest::get("https://api.exchangeratesapi.io/latest?base=USD")?.text()?;
    //let map: serde_json::Value = serde_json::from_str(&resp_text)?;
    //println!("{:#?}", map);

    // Save map as a json file
    Ok(())
}

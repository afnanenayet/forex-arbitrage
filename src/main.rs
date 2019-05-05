use acquisition::{construct_graph, load_graph, save_graph};
use failure::{self, Error, Fail};
use reqwest;
use std::collections::HashMap;
use structopt::StructOpt;

mod acquisition;
mod graph;

#[derive(Debug, Fail)]
/// Errors that can stem from supplying improper arguments at the command line
enum ArgumentError {
    #[fail(display = "`graph_file` and `save_file` were both defined, which is illegal")]
    /// This error is invoked because only `graph_file` or `save_file` can be defined, but not both
    FileConflict,
}

/// Command line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "forex-arbitrage",
    about = "Graph based forex arbitrage detection"
)]
struct Opt {
    /// (optional) The path to a serialized graph file
    #[structopt(short = "i", long = "input")]
    graph_file: Option<String>,

    /// (optional) The file name to save the serialized graph. If no filename is supplied, this
    /// will use the default filename format: "$UNIX_EPOCH-forex-graph.json"
    #[structopt(short = "o", long = "output")]
    save_file: Option<String>,
}

impl Opt {
    /// Verify that the opts are in a valid configuration
    pub fn verify(&self) -> Result<(), Error> {
        // Either `graph_file` or `save_file` can be set, or neither, but never both
        if self.graph_file.is_some() && self.save_file.is_some() {
            return Err(Error::from(ArgumentError::FileConflict));
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    opt.verify()?;

    // Determine whether the graph needs to be constructed or loaded from a file
    let map = match opt.graph_file {
        Some(fname) => load_graph(&fname)?,
        None => {
            let mut client = reqwest::Client::new();
            let map = construct_graph(&mut client, "USD")?;
            save_graph(&map, opt.save_file)?;
            map
        }
    };

    let arbitrage_op = graph::detect_any_cycle(&map);

    if arbitrage_op.values().any(|x| *x) {
        println!("arbitrage opportunities detected:");

        for start_currency in map.keys() {
            for end_currency in map[start_currency].keys() {
                if map[start_currency][end_currency] < 0.0 {
                    println!(
                        "- {} -> {} -> {}",
                        start_currency, end_currency, start_currency
                    );
                }
            }
        }
    } else {
        println!("no arbitrage opportunities detected");
    }
    Ok(())
}

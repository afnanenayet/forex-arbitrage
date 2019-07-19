use acquisition::{construct_graph, load_graph, save_graph, transform_graph};
use failure::{self, Error, Fail};
use reqwest;
use std::collections::{HashMap, HashSet};
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
    let rates = if let Some(fname) = opt.graph_file {
        load_graph(&fname)?
    } else {
        let mut client = reqwest::Client::new();
        let map = construct_graph(&mut client, "USD")?;
        save_graph(&map, opt.save_file)?;
        map
    };
    println!("--> Data acquired");
    let transformed_rates = transform_graph(&rates);
    println!("--> Data transformed");
    let arbitrage_op = graph::detect_any_cycle(&transformed_rates);
    println!("--> Graph has been processed");

    // Filter out parts of the map that returned `None`, since they aren't useful to us
    let valid_arbitrage_ops: HashSet<Vec<&String>> = arbitrage_op
        .iter()
        .filter_map(|(_k, v)| (*v).clone())
        .collect();

    if valid_arbitrage_ops.is_empty() {
        println!("no arbitrage opportunities detected");
    } else {
        println!("arbitrage opportunities detected:");

        for path in valid_arbitrage_ops {
            let currency_path = path;
            let mut res: graph::Rate = 1.0;

            for i in 0..currency_path.len() - 1 {
                res *= rates[currency_path[i]][currency_path[i + 1]];
            }

            if res > 1.0 {
                for currency in currency_path.iter().take(currency_path.len() - 1) {
                    print!("{} -> ", currency);
                }
                print!("{}", currency_path[currency_path.len() - 1]);
                println!(" (gain: x{})", res);
            }
        }
    }
    Ok(())
}

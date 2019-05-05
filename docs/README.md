# forex arbitrage

## Summary

This program is a proof of concept showing how you can detect arbitrage
opportunities using forex pricing data. It uses pricing data from
exchangeratesapi.io (free) to acquire the pricing data.

The data is taken and converted into a graph, with the weights being the
negative logarithm of the pricing data. The negative log is taken because I am
using the Bellman-Ford algorithm to detect arbitrage opportunities. The
Bellman-Ford algorithm detects negative cycles, but we want a situation where
the pricing data leads to a _positive_ cycle of prices, such that we can start
at some currency X, and perform some set of currency conversions such that we
end up at currency X, but with more money than we started from.

Because the Bellman-Ford algorithm adds weights, we need to figure out some
transformation that allows us to use addition to find a positive weight cycle.
We know that `log(a) + log(b) = log(a * b)`, which takes care of the addition
problem. Using `-log(x)` lets us turn positive cycles into negative ones, which
means that we can detect arbitrage opportunities by finding a negative cycle in
a graph.

## Usage

The program saves the generated conversion rate graph as a json file. You can
change the name of the json file from the default naming scheme using the `-o`
flag. You can also elect to load pricing data from a json file using the `-i`
flag, if you're just messing around or debugging and don't need the most recent
data.

## Development

Compile this program with:

```sh
cargo build

# or

cargo build --release
```

All files are formatted using `rustfmt`.

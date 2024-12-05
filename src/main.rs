use std::io::BufRead;

use clap::Parser;

use aoc_2024 as lib;

#[derive(Parser)]
/// Advent of Code 2024 solutions by @akosnad
struct Cli {
    /// Which day's solution to run
    day: u8,

    /// Input file to use (if not provided, read from stdin)
    input: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input = match cli.input {
        Some(path) => std::fs::read_to_string(path)?
            .split('\n')
            .map(String::from)
            .collect(),
        None => std::io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<_>, _>>()?,
    };

    match cli.day {
        1 => lib::day1::run(input),
        _ => anyhow::bail!("Day {} not implemented yet!", cli.day),
    }
}

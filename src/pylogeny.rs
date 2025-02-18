use crate::graph::additive_phylogeny;
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct PhylogenyArgs {
    #[arg(long, required = true, value_name = "file1")]
    pub input: String,
}

pub fn run_phylogeny(args: PhylogenyArgs) -> Result<(), Box<dyn Error>> {
    // Read file contents
    let contents = fs::read_to_string(&args.input)?;
    let mut lines = contents.lines();

    // Skip header line
    lines.next();

    // Parse the distance matrix more efficiently
    let matrix: Vec<Vec<usize>> = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();

    // Compute phylogeny
    let mut graph = additive_phylogeny(&matrix, &mut matrix.len())?;

    let mut keys = graph.keys().copied().collect::<Vec<_>>();
    keys.sort();
    for v in graph.values_mut() {
        v.sort();
    }
    // Output results
    for node in keys.iter() {
        for (neighbor, distance) in graph.get(node).unwrap() {
            println!("{}->{}:{}", node, neighbor, distance);
        }
    }

    Ok(())
}

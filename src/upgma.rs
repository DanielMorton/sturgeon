use crate::graph::upgma;
use crate::pylogeny::PhylogenyArgs;
use std::error::Error;
use std::fs;

pub fn run_upgma(args: PhylogenyArgs) -> Result<(), Box<dyn Error>> {
    // Read file contents
    let contents = fs::read_to_string(&args.input)?;
    let mut lines = contents.lines();

    lines.next();

    // Parse the distance matrix more efficiently
    let matrix = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect::<Vec<_>>();

    // Compute phylogeny
    let graph = upgma(&matrix)?;

    let mut keys = graph.keys().copied().collect::<Vec<_>>();
    keys.sort();
    // Output results
    for node in keys.iter() {
        for (neighbor, distance) in graph.get(node).unwrap() {
            println!("{}->{}:{:.3}", node, neighbor, distance);
        }
    }

    Ok(())
}

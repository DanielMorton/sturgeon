use std::collections::{HashMap, HashSet};
use std::error::Error;

fn overlap_graph(patterns: &[&str]) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let p = patterns[0].len();

    // Create initial graph structure using HashMaps
    let mut graph = HashMap::new();
    for &pattern in patterns {
        graph.insert(pattern, HashSet::new());
    }

    // Build edges based on suffix-prefix matches
    for &pattern in patterns {
        for (key, value) in graph.iter_mut() {
            // Check if suffix of key matches prefix of pattern
            if key[1..] == pattern[..p - 1] {
                value.insert(pattern);
            }
        }
    }

    // Convert to final edge list format
    let mut kmer_graph = HashMap::new();
    for (key, value) in graph {
        if !value.is_empty() {
            let edges = value.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
            kmer_graph.insert(key.to_owned(), edges);
        }
    }
    Ok(kmer_graph)
}

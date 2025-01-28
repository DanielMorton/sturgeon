use crate::graph::graph::Graph;
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn overlap_graph(patterns: &[String]) -> Result<Graph<String>, Box<dyn Error>> {
    let p = patterns[0].len();

    // Create initial graph structure using HashMaps
    let mut graph = HashMap::new();
    for pattern in patterns {
        graph.insert(pattern.to_owned(), HashSet::new());
    }

    // Build edges based on suffix-prefix matches
    for pattern in patterns {
        for (key, value) in graph.iter_mut() {
            // Check if suffix of key matches prefix of pattern
            if key[1..] == pattern[..p - 1] {
                value.insert(pattern.to_owned());
            }
        }
    }
    // Convert to final edge list format
    let mut kmer_graph = Graph::new();
    for (key, value) in graph {
        if !value.is_empty() {
            let mut edges = value.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
            edges.sort();
            kmer_graph.insert(key.to_owned(), edges);
        }
    }
    Ok(kmer_graph)
}

mod tests {
    use crate::graph::overlap::overlap_graph;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_overlap_graph1() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("AAG"),
            format!("AGA"),
            format!("ATT"),
            format!("CTA"),
            format!("CTC"),
            format!("GAT"),
            format!("TAC"),
            format!("TCT"),
            format!("TCT"),
            format!("TTC"),
        ];
        let ans = HashMap::from([
            (format!("AAG"), vec![format!("AGA")]),
            (format!("AGA"), vec![format!("GAT")]),
            (format!("ATT"), vec![format!("TTC")]),
            (format!("CTA"), vec![format!("TAC")]),
            (format!("CTC"), vec![format!("TCT")]),
            (format!("GAT"), vec![format!("ATT")]),
            (format!("TCT"), vec![format!("CTA"), format!("CTC")]),
            (format!("TTC"), vec![format!("TCT")]),
        ]);
        assert_eq!(overlap_graph(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_overlap_graph2() -> Result<(), Box<dyn Error>> {
        let patterns = vec![format!("ACT"), format!("CTT"), format!("TTT")];
        let ans = HashMap::from([
            (format!("ACT"), vec![format!("CTT")]),
            (format!("CTT"), vec![format!("TTT")]),
            (format!("TTT"), vec![format!("TTT")]),
        ]);
        assert_eq!(overlap_graph(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_overlap_graph3() -> Result<(), Box<dyn Error>> {
        let patterns = vec![format!("CCCC")];
        let ans = HashMap::from([(format!("CCCC"), vec![format!("CCCC")])]);
        assert_eq!(overlap_graph(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_overlap_graph4() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("CT"),
            format!("TT"),
            format!("TT"),
            format!("TT"),
            format!("TT"),
            format!("TT"),
        ];
        let ans = HashMap::from([
            (format!("CT"), vec![format!("TT")]),
            (format!("TT"), vec![format!("TT")]),
        ]);
        assert_eq!(overlap_graph(&patterns)?, ans);
        Ok(())
    }
}

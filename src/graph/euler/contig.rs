use std::collections::HashSet;
use std::error::Error;
use crate::graph::debruijn::debruijn_kmers;
use crate::graph::graph::{Contig, Graph};
use crate::graph::reconstruction::genome_path;


pub fn maximal_non_branching_paths<T>(graph: &Graph<T>) -> Result<Vec<Contig<T>>, Box<dyn Error>>
    where T: Clone + Eq + std::hash::Hash {
    let mut paths: Vec<Contig<T>> = Vec::new();
    let mut visited = HashSet::new();

    // Helper function to check if a node is 1-in-1-out
    non_isolate_paths(graph, &mut paths, &mut visited)?;

    isolate_cycles(graph, &mut paths, &mut visited)?;

    Ok(paths)
}

fn is_one_in_out<T>(graph: &Graph<T>, node: &T) -> Result<bool, Box<dyn Error>>
    where T: Clone + Eq + std::hash::Hash {
    let in_degree = graph.values()
        .flat_map(|edges| edges.iter())
        .filter(|&n| n == node)
        .count();
    let out_degree = graph.get(node).map_or(0, |edges| edges.len());
    Ok(in_degree == 1 && out_degree == 1)
}

fn non_isolate_paths<T>(graph: &Graph<T>,
                        paths: &mut Vec<Contig<T>>,
                        visited: &mut HashSet<T>) -> Result<(), Box<dyn Error>>
    where T: Clone + Eq + std::hash::Hash {
    // Process non-isolate paths
    for start_node in graph.keys() {
        if !is_one_in_out(graph, start_node)? {
            if let Some(edges) = graph.get(start_node) {
                for next_node in edges {
                    let mut contig = Contig::new();
                    contig.push(start_node.clone());
                    contig.push(next_node.clone());
                    visited.insert(start_node.clone());

                    // Extend path while finding 1-in-1-out nodes
                    let mut current = next_node;
                    while is_one_in_out(graph, current)? {
                        visited.insert(current.clone());
                        if let Some(next_edges) = graph.get(current) {
                            let next = &next_edges[0];
                            contig.push(next.clone());
                            current = next;
                        }
                    }
                    paths.push(contig);
                }
            }
        }
    }
    Ok(())
}

fn isolate_cycles<T>(graph: &Graph<T>,
                  paths: &mut Vec<Contig<T>>,
                  visited: &mut HashSet<T>) -> Result<(), Box<dyn Error>>
    where T: Clone + Eq + std::hash::Hash{
    // Find isolated cycles
    for node in graph.keys() {
        if !visited.contains(node) {
            let mut cycle = vec![node.clone()];
            let mut current = node;
            visited.insert(node.clone());

            while let Some(edges) = graph.get(current) {
                if edges.is_empty() {
                    break;
                }
                let next = &edges[0];
                if next == node {
                    // Complete the cycle
                    cycle.push(next.clone());
                    paths.push(cycle);
                    break;
                }
                if visited.contains(next) {
                    break;
                }
                cycle.push(next.clone());
                visited.insert(next.clone());
                current = next;
            }
        }
    }
    Ok(())
}

pub fn contig_generation(patterns: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let graph = debruijn_kmers(patterns)?;
    let mut contigs = maximal_non_branching_paths(&graph)?.iter()
        .map(|contig| genome_path(contig)).collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    contigs.sort();
    Ok(contigs)
}

mod tests {
    use std::error::Error;
    use crate::graph::euler::contig::contig_generation;

    #[test]
    fn test_contig_generation1() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("ATG"), format!("ATG"),
            format!("TGT"), format!("TGG"),
            format!("CAT"), format!("GGA"),
            format!("GAT"), format!("AGA"),
        ];
        assert_eq!(contig_generation(&path)?, vec!["AGA", "ATG", "ATG", "CAT", "GAT", "TGGA", "TGT"]);
        Ok(())
    }

    #[test]
    fn test_contig_generation2() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("GTT"), format!("TTA"),
            format!("TAC"), format!("TTT"),
        ];
        assert_eq!(contig_generation(&path)?, vec!["GTT", "TTAC", "TTT"]);
        Ok(())
    }

    #[test]
    fn test_contig_generation3() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("AG"), format!("GT"),
            format!("GC"), format!("TA")
        ];
        assert_eq!(contig_generation(&path)?, vec!["GC", "GTAG"]);
        Ok(())
    }

    #[test]
    fn test_contig_generation4() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("GAGA"), format!("AGAG"),
            format!("AACG"), format!("ACGT"),
            format!("ACGG")
        ];
        assert_eq!(contig_generation(&path)?, vec!["AACG", "ACGG", "ACGT", "GAGAG"]);
        Ok(())
    }

    #[test]
    fn test_contig_generation5() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("TGAG"), format!("GACT"),
            format!("CTGA"), format!("ACTG"),
            format!("CTGA")
        ];
        assert_eq!(contig_generation(&path)?, vec!["CTGA", "CTGA", "GACTG", "TGAG"]);
        Ok(())
    }
}
use crate::genome::breakpoints::make_breakpoints_graph;
use crate::genome::chromosome::{colored_edges, cycle_to_chromosome, sort_genome};
use crate::genome::cluster::{group_nodes, group_nodes_pairs};
use crate::genome::two_break::two_break_on_graph;
use std::collections::{HashMap, HashSet};
use std::error::Error;

/// Group nodes using disjoint set (union-find) data structure

/// Build edge dictionary for genome rearrangement
fn build_edge_dict(
    edges: &[(i32, i32)],
    node_parents: &HashMap<i32, i32>,
) -> Result<HashMap<i32, HashMap<i32, i32>>, Box<dyn Error>> {
    let mut edge_dict: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    for &(a, b) in edges {
        let id = *node_parents.get(&a).unwrap();
        edge_dict.entry(id).or_default().insert(a, b);
        edge_dict.entry(id).or_default().insert(b, a);
    }

    Ok(edge_dict)
}

/// Perform two-break operation on genome
fn get_new_genome(edges: &[(i32, i32)]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let nodes_id = group_nodes_pairs(&edges)?;
    let mut edge_dict = build_edge_dict(&edges, &nodes_id)?;

    let mut nodes_dict = HashMap::new();

    for (id, e_dict) in edge_dict.iter_mut() {
        let mut curr_nodes = Vec::new();

        let mut curr_node0 = *e_dict.keys().next().unwrap();

        while !e_dict.is_empty() {
            curr_nodes.push(curr_node0);

            let curr_node1 = curr_node0 + curr_node0 % 2 * 2 - 1;

            curr_nodes.push(curr_node1);

            let new_node = e_dict[&curr_node1];
            e_dict.remove(&curr_node0);
            e_dict.remove(&curr_node1);

            curr_node0 = new_node;
        }

        nodes_dict.insert(*id, curr_nodes);
    }

    let mut new_genome = nodes_dict
        .into_values()
        .map(|nodes| cycle_to_chromosome(&nodes))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    new_genome.sort_by_key(|x| x[0].abs());
    Ok(new_genome)
}

/// Find edge from nontrivial cycle in breakpoint graph
fn edge_from_nontrivial_cycle(
    edges: &[(i32, i32)],
    red_edges: &[(i32, i32)],
    blue_edges: &[(i32, i32)],
) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    // Get node groupings
    let node_parents = group_nodes(edges)?;

    // Early return if cycles match blocks
    if node_parents.values().collect::<HashSet<_>>().len() == blue_edges.len() {
        return Ok(Vec::new());
    }

    let edge_dict = build_edge_dict(&edges, &node_parents)?;
    let red_edge_dict = build_edge_dict(&red_edges, &node_parents)?;

    // Process edges and find first valid blue edge
    let (edge, id) = blue_edges
        .iter()
        .find_map(|&(a, b)| {
            let id = *node_parents.get(&a).unwrap();

            // Insert into edge_dict
            let edge_map = edge_dict.get(&id).unwrap();

            // Return Some(FoundEdge) if this is a valid blue edge
            if edge_map.len() > 2 && blue_edges.contains(&(a, b)) {
                Some(((a, b), id))
            } else {
                None
            }
        })
        .unwrap();

    let red_edges_map = red_edge_dict.get(&id).unwrap();

    let edges_to_remove = vec![
        (edge.0, *red_edges_map.get(&edge.0).unwrap()),
        (edge.1, *red_edges_map.get(&edge.1).unwrap()),
    ];

    Ok(edges_to_remove)
}

/// Find shortest rearrangement between two genomes
pub fn two_break_sorting(
    p: &[Vec<i32>],
    q: &[Vec<i32>],
) -> Result<Vec<Vec<Vec<i32>>>, Box<dyn Error>> {
    let mut result = vec![p.to_vec()];

    let mut red_edges = colored_edges(p)?;
    let blue_edges = colored_edges(q)?;
    let mut breakpoint_graph = make_breakpoints_graph(&red_edges, &blue_edges)?;

    let mut edges_to_remove =
        edge_from_nontrivial_cycle(&breakpoint_graph, &red_edges, &blue_edges)?;

    let mut p_copy;
    while !edges_to_remove.is_empty() {
        two_break_on_graph(&mut red_edges, &edges_to_remove)?;

        breakpoint_graph = make_breakpoints_graph(&red_edges, &blue_edges)?;

        p_copy = get_new_genome(&red_edges)?;
        let sorted = sort_genome(&p_copy)?;

        result.push(sorted);

        edges_to_remove = edge_from_nontrivial_cycle(&breakpoint_graph, &red_edges, &blue_edges)?;
    }

    Ok(result)
}

mod tests {
    use crate::genome::path::two_break_sorting;
    use std::error::Error;

    #[test]
    fn test_two_break_sorting1() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5, 6]];
        let q = vec![vec![1, -3, -6, -5], vec![2, -4]];
        let ans = vec![
            vec![vec![1, 2, 3, 4, 5, 6]],
            vec![vec![1, -3, -2, 4, 5, 6]],
            vec![vec![1, -3, 5, 6], vec![2, -4]],
            vec![vec![1, -3, -6, -5], vec![2, -4]],
        ];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_sorting2() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];
        let q = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];
        let ans = vec![vec![vec![1, 2, 3, 4, 5], vec![6, 7]]];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_sorting3() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, -2, -3, 4]];
        let q = vec![vec![1, 2, -3, 4]];
        let ans = vec![vec![vec![1, -2, -3, 4]], vec![vec![1, 2, -3, 4]]];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_sorting4() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3], vec![-4, -5]];
        let q = vec![vec![-1, -2, -3], vec![4, 5]];
        let ans = vec![
            vec![vec![1, 2, 3], vec![-4, -5]],
            vec![vec![1, 2], vec![3], vec![4, 5]],
            vec![vec![1, 3, 2], vec![4, 5]],
        ];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_sortinge5() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3], vec![4, 5]];
        let q = vec![vec![2, -1, 4, -3, 5]];
        let ans = vec![
            vec![vec![1, 2, 3], vec![4, 5]],
            vec![vec![1, 2, 3, -5, -4]],
            vec![vec![1, -2, 3, -5, -4]],
            vec![vec![1, -2, -4], vec![3, -5]],
            vec![vec![1, -2, -5, 3, -4]],
        ];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }
}

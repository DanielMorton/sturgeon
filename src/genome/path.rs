use crate::genome::chromosome::{colored_edges, cycle_to_chromosome, sort_genome};
use crate::genome::two_break::two_break_on_graph;
use crate::utils::{find_parent, union};
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn group_nodes(
    edges: &HashSet<(i32, i32)>,
) -> Result<(HashSet<i32>, HashMap<i32, i32>), Box<dyn Error>> {
    let mut parent = HashMap::new();
    let mut rank = HashMap::new();

    // Initialize parent and rank in a single pass
    for &(a, b) in edges {
        for &node in &[a, b] {
            parent.entry(node).or_insert(node);
            rank.entry(node).or_insert(0);
        }
    }

    // Process unions
    for &(a, b) in edges {
        // Connect nodes in edge
        union(a, b, &mut parent, &mut rank)?;
    }

    let mut nodes_id = HashMap::new();
    let mut nodes_sets = HashSet::new();

    let parent_keys = parent.keys().into_iter().map(|&x| x).collect::<Vec<_>>();

    for node in parent_keys {
        let id = find_parent(node, &mut parent)?;
        nodes_id.insert(node, id);
        nodes_sets.insert(id);
    }

    Ok((nodes_sets, nodes_id))
}
/// Group nodes using disjoint set (union-find) data structure
fn group_nodes_pairs(
    edges: &HashSet<(i32, i32)>,
) -> Result<(HashSet<i32>, HashMap<i32, i32>), Box<dyn Error>> {
    let mut parent = HashMap::new();
    let mut rank = HashMap::new();

    // Initialize parent and rank in a single pass
    for &(a, b) in edges {
        for &node in &[a, b] {
            parent.entry(node).or_insert(node);
            rank.entry(node).or_insert(0);
        }
    }

    // Process unions
    for &(a, b) in edges {
        // Connect nodes in edge
        union(a, b, &mut parent, &mut rank)?;

        // Connect odd/even pairs
        let a_pair = a + (a % 2 * 2 - 1);
        let b_pair = b + (b % 2 * 2 - 1);
        println!("a {} a_pair {}", a, a_pair);
        println!("b {} b_pair {}", b, b_pair);
        union(a, a_pair, &mut parent, &mut rank)?;
        union(b, b_pair, &mut parent, &mut rank)?;
    }

    let mut nodes_id = HashMap::new();
    let mut nodes_sets = HashSet::new();

    let parent_keys = parent.keys().copied().collect::<Vec<_>>();

    for node in parent_keys {
        let id = find_parent(node, &mut parent)?;
        nodes_id.insert(node, id);
        nodes_sets.insert(id);
    }

    Ok((nodes_sets, nodes_id))
}

/// Build edge dictionary for genome rearrangement
fn build_edge_dict(
    edges: &HashSet<(i32, i32)>,
    nodes_id: &HashMap<i32, i32>,
) -> Result<HashMap<i32, HashMap<i32, i32>>, Box<dyn Error>> {
    let mut edge_dict: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    for &(a, b) in edges {
        let id = *nodes_id.get(&a).unwrap();
        edge_dict.entry(id).or_default().insert(a, b);
        edge_dict.entry(id).or_default().insert(b, a);
    }

    Ok(edge_dict)
}

/// Perform two-break operation on genome
fn two_break_on_genome(edges: &HashSet<(i32, i32)>) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    println!("Edges = {:?}", edges);
    let (_, nodes_id) = group_nodes_pairs(&edges)?;
    let edge_dict = build_edge_dict(&edges, &nodes_id)?;
    println!("Edge Dict = {:?}", edge_dict);

    let mut nodes_dict: HashMap<i32, Vec<i32>> = HashMap::new();

    for (id, e_dict) in edge_dict.iter() {
        let mut curr_nodes = Vec::new();
        let mut e_dict = e_dict.clone();

        let mut curr_node0 = *e_dict.keys().next().unwrap();

        while !e_dict.is_empty() {
            curr_nodes.push(curr_node0);

            let curr_node1 = if curr_node0 % 2 == 1 {
                curr_node0 + 1
            } else {
                curr_node0 - 1
            };
            println!("Node 0 = {} Node 1 = {}", curr_node0, curr_node1);

            curr_nodes.push(curr_node1);

            let new_node = e_dict[&curr_node1];
            e_dict.remove(&curr_node0);
            e_dict.remove(&curr_node1);

            curr_node0 = new_node;
        }

        nodes_dict.insert(*id, curr_nodes);
    }
    println!("Nodes Dict {:?}", nodes_dict);

    let mut new_genome = nodes_dict
        .into_values()
        .map(|nodes| cycle_to_chromosome(&nodes))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    new_genome.sort_by_key(|x| x[0].abs());
    println!("New Genome {:?}", new_genome);
    Ok(new_genome)
}

/// Find edge from nontrivial cycle in breakpoint graph
fn edge_from_nontrivial_cycle(
    edges: &HashSet<(i32, i32)>,
    red_edges: &HashSet<(i32, i32)>,
    blue_edges: &HashSet<(i32, i32)>,
    blocks: usize,
) -> Result<(bool, Vec<(i32, i32)>), Box<dyn Error>> {
    // Get node groupings
    let (nodes_set, nodes_id) = group_nodes(edges)?;
    println!("Node Set {:?}", nodes_set);
    println!("Nodes ID {:?}", nodes_id);

    // Early return if cycles match blocks
    if nodes_set.len() == blocks {
        return Ok((false, Vec::with_capacity(0)));
    }

    // Pre-allocate dictionaries with estimated capacities
    let mut edge_dict = HashMap::new();
    let mut red_edge_dict = HashMap::new();

    println!("Edges {:?}", edges);
    edges.iter().for_each(|&(a, b)| {
        let id = *nodes_id.get(&a).unwrap();

        let edges_for_id = edge_dict.entry(id).or_insert_with(|| HashMap::new());
        edges_for_id.insert(a, b);
        edges_for_id.insert(b, a);

        if red_edges.contains(&(a, b)) {
            let red_edges_for_id = red_edge_dict.entry(id).or_insert_with(|| HashMap::new());
            red_edges_for_id.insert(a, b);
            red_edges_for_id.insert(b, a);
        }
    });

    // Structure to track valid edges
    #[derive(Debug)]
    struct FoundEdge {
        edge: (i32, i32),
        id: i32,
    }

    // Process edges and find first valid blue edge
    let found_edge = blue_edges.iter().find_map(|&(a, b)| {
        let id = *nodes_id.get(&a).unwrap();

        // Insert into edge_dict
        let edges_for_id = edge_dict.get(&id).unwrap();

        // Return Some(FoundEdge) if this is a valid blue edge
        if edges_for_id.len() > 2 && blue_edges.contains(&(a, b)) {
            Some(FoundEdge { edge: (a, b), id })
        } else {
            None
        }
    });

    println!("Found edge {:?}", found_edge);
    // Extract found edge and construct result
    let FoundEdge { edge, id } = found_edge.unwrap();

    let red_edges_for_id = red_edge_dict.get(&id).unwrap();

    println!("{:?}", red_edges_for_id);
    println!("{:?}", edge);
    let removed_red_edges = vec![
        (edge.0, *red_edges_for_id.get(&edge.0).unwrap()),
        (edge.1, *red_edges_for_id.get(&edge.1).unwrap()),
    ];

    Ok((true, removed_red_edges))
}

/// Find shortest rearrangement between two genomes
pub fn two_break_sorting(
    p: &[Vec<i32>],
    q: &[Vec<i32>],
) -> Result<Vec<Vec<Vec<i32>>>, Box<dyn Error>> {
    let blocks = p.iter().map(|a| a.len()).sum();
    let mut result = vec![p.to_vec()];

    let mut red_edges = colored_edges(p)?;
    let blue_edges = colored_edges(q)?;
    let mut breakpoint_graph = red_edges.union(&blue_edges).cloned().collect();
    println!("{:?}", breakpoint_graph);

    println!("Red edges {:?}", red_edges);
    println!("Blue edges {:?}", blue_edges);
    let (mut has_nontrivial_cycle, mut removed_red_edges) =
        edge_from_nontrivial_cycle(&breakpoint_graph, &red_edges, &blue_edges, blocks)?;
    println!("Removed Edges {:?}", removed_red_edges);

    let mut p_copy;
    while has_nontrivial_cycle {
        two_break_on_graph(&mut red_edges, &removed_red_edges)?;

        breakpoint_graph = red_edges.union(&blue_edges).cloned().collect();

        p_copy = two_break_on_genome(&red_edges)?;
        let sorted = sort_genome(&p_copy)?;

        result.push(sorted);

        (has_nontrivial_cycle, removed_red_edges) =
            edge_from_nontrivial_cycle(&breakpoint_graph, &red_edges, &blue_edges, blocks)?;
        println!("Removed Edges {:?}", removed_red_edges);
    }

    Ok(result)
}

mod tests {
    use crate::genome::path::two_break_sorting;
    use std::error::Error;

    #[test]
    fn test_two_break_distance1() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5, 6]];
        let q = vec![vec![1, -3, -6, -5], vec![2, -4]];
        let ans = vec![
            vec![vec![1, 2, 3, 4, 5, 6]],
            vec![vec![1, 2, 3, 4, -6, -5]],
            vec![vec![1, 2, -4, -3, -6, -5]],
            vec![vec![1, -3, -6, -5], vec![2, -4]],
        ];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_distance2() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];
        let q = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];
        let ans = vec![vec![vec![1, 2, 3, 4, 5], vec![6, 7]]];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }

    #[test]
    fn test_two_break_distance3() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, -2, -3, 4]];
        let q = vec![vec![1, 2, -3, 4]];
        let ans = vec![vec![vec![1, -2, -3, 4]], vec![vec![1, 2, -3, 4]]];

        assert_eq!(two_break_sorting(&p, &q)?, ans);
        Ok(())
    }
}

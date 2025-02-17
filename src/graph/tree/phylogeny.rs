use crate::graph::tree::dfs::{find_edge_to_split, find_node_at_distance, find_path};
use crate::graph::tree::distance::{calculate_limb_length, find_distance};
use crate::utils::{add_weighted_edge_pair, WeightedGraph};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

fn find_splitting_pair(d: &[Vec<usize>]) -> Result<(usize, usize, usize), Box<dyn Error>> {
    let d_tail = d.last().unwrap();
    for (i, row) in d.iter().enumerate().take(d.len() - 1) {
        for k in i + 1..d.len() - 1 {
            let row_tail = row.last().unwrap();
            if row_tail + d_tail[k] == row[k] {
                return Ok((i, k, *row_tail));
            }
        }
    }
    panic!("No splitting pair found");
}

fn add_node_at_distance(
    graph: &mut WeightedGraph<usize, usize>,
    inner_vertex: &mut usize,
    start: usize,
    end: usize,
    target_distance: usize,
) -> Result<usize, Box<dyn Error>> {
    let v = *inner_vertex;
    *inner_vertex += 1;

    // Find the edge to split
    let path = find_path(graph, start, end)?;
    let (u, w) = find_edge_to_split(graph, &path, target_distance)?;

    // Remove old edge
    let edge_weight = graph
        .get(&u)
        .unwrap()
        .iter()
        .find(|&&(node, _)| node == w)
        .unwrap()
        .1;

    graph.get_mut(&u).unwrap().retain(|&(node, _)| node != w);
    graph.get_mut(&w).unwrap().retain(|&(node, _)| node != u);

    // Add new node and edges
    let dist_i_u = find_distance(graph, start, u)?;
    let _ = add_weighted_edge_pair(graph, u, v, target_distance - dist_i_u);
    let _ = add_weighted_edge_pair(graph, v, w, edge_weight - (target_distance - dist_i_u));
    Ok(v)
}

fn additive_phylogeny(
    matrix: &[Vec<usize>],
    inner_vertex: &mut usize,
) -> Result<WeightedGraph<usize, usize>, Box<dyn Error>> {
    // Base case: tree with two nodes
    if matrix.len() == 2 {
        return Ok(HashMap::from([
            (0, vec![(1, matrix[0][1])]),
            (1, vec![(0, matrix[1][0])]),
        ]));
    }
    let matrix_len = matrix.len();

    // Calculate limb length for the last leaf
    let limb_length = calculate_limb_length(matrix, matrix_len - 1)?;

    // Create a copy of the distance matrix and adjust distances
    let mut matrix_prime = matrix.to_vec();
    for row in matrix_prime.iter_mut().take(matrix_len - 1) {
        row[matrix_len - 1] -= limb_length;
    }

    for j in 0..matrix_len - 1 {
        matrix_prime[matrix_len - 1][j] = matrix_prime[j][matrix_len - 1];
    }

    // Find i,k such that Di,k = Di,n + Dn,k
    let (start, end, distance) = find_splitting_pair(&matrix_prime)?;

    // Remove last row and column to create smaller matrix
    matrix_prime = matrix_prime
        .iter()
        .take(matrix_len - 1)
        .map(|row| row.iter().take(row.len() - 1).copied().collect())
        .collect();

    // Recursive call
    let mut graph = additive_phylogeny(&matrix_prime, inner_vertex)?;

    let v = match find_node_at_distance(&graph, start, distance)? {
        Some(v) => v,
        None => add_node_at_distance(&mut graph, inner_vertex, start, end, distance)?,
    };

    // Add the new leaf
    let _ = add_weighted_edge_pair(&mut graph, v, matrix_len - 1, limb_length);

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use crate::graph::tree::phylogeny::additive_phylogeny;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_additive_phylogeny1() -> Result<(), Box<dyn Error>> {
        let matrix = vec![
            vec![0, 13, 21, 22],
            vec![13, 0, 12, 13],
            vec![21, 12, 0, 13],
            vec![22, 13, 13, 0],
        ];
        let mut graph = additive_phylogeny(&matrix, &mut matrix.len())?;
        for v in graph.values_mut() {
            v.sort();
        }
        let mut ans = HashMap::from([
            (0, vec![(4, 11)]),
            (1, vec![(4, 2)]),
            (2, vec![(5, 6)]),
            (3, vec![(5, 7)]),
            (4, vec![(0, 11), (1, 2), (5, 4)]),
            (5, vec![(4, 4), (3, 7), (2, 6)]),
        ]);
        for v in ans.values_mut() {
            v.sort();
        }
        assert_eq!(graph, ans);
        Ok(())
    }
}

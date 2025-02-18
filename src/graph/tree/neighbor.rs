/*use std::collections::HashMap;
use std::error::Error;
use crate::utils::WeightedGraph;

fn neighbor_joining_matrix(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {

}

pub fn neighbor_joining(
    matrix: &[Vec<f64>],
    inner_vertex: &mut usize,
) -> Result<WeightedGraph<usize, f64>, Box<dyn Error>> {
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
}*/
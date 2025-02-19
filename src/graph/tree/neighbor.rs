use crate::graph::tree::matrix::{
    find_minimal_coordinates, new_neigbhor_join_row, row_sum, update_distance_matrix,
};
use crate::utils::{add_weighted_edge_pair, WeightedGraph};
use std::error::Error;

fn neighbor_joining_matrix(
    matrix: &[Vec<f64>],
    total_distance: &[f64],
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let matrix_len = matrix.len();
    let mut matrix_nj = matrix.to_vec();
    let d = matrix_len as f64 - 2.0;

    for (i, row) in matrix_nj.iter_mut().enumerate() {
        for j in 0..matrix_len {
            row[j] *= d;
            if i != j {
                row[j] -= total_distance[i];
                row[j] -= total_distance[j];
            }
        }
    }

    Ok(matrix_nj)
}

pub fn neighbor_joining(matrix: &[Vec<f64>]) -> Result<WeightedGraph<usize, f64>, Box<dyn Error>> {
    let mut graph = WeightedGraph::new();
    let mut matrix_prime = matrix.to_vec();

    // Initialize clusters
    let mut clusters: Vec<usize> = (0..matrix.len()).collect();

    // Create a map to keep track of cluster sizes
    let mut next_index = matrix.len();

    while clusters.len() > 2 {
        let total_distance = row_sum(&matrix_prime)?;
        let matrix_nj = neighbor_joining_matrix(&mut matrix_prime, &total_distance)?;
        let (i, j) = find_minimal_coordinates(&matrix_nj)?;
        let ci = clusters[i];
        let cj = clusters[j];

        let delta = (total_distance[i] - total_distance[j]) / (matrix_prime.len() as f64 - 2.0);
        let length_i = (matrix_prime[i][j] + delta) / 2.0;
        let length_j = (matrix_prime[i][j] - delta) / 2.0;
        let _ = add_weighted_edge_pair(&mut graph, next_index, ci, length_i);
        let _ = add_weighted_edge_pair(&mut graph, next_index, cj, length_j);

        let new_row = new_neigbhor_join_row(&matrix_prime, i, j)?;
        let _ = update_distance_matrix(&mut matrix_prime, i, j, &new_row)?;
        clusters.remove(j);
        clusters.remove(i);
        clusters.push(next_index);
        next_index += 1;
    }
    let _ = add_weighted_edge_pair(&mut graph, next_index - 1, next_index - 2, matrix_prime[0][1])?;

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use crate::graph::tree::neighbor::neighbor_joining;
    use approx::abs_diff_eq;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_neighbor_joining1() -> Result<(), Box<dyn Error>> {
        let matrix = vec![
            vec![0.0, 23.0, 27.0, 20.0],
            vec![23.0, 0.0, 30.0, 28.0],
            vec![27.0, 30.0, 0.0, 30.0],
            vec![20.0, 28.0, 30.0, 0.0],
        ];
        let graph = neighbor_joining(&matrix)?;
        let ans = HashMap::from([
            (0, vec![(4, 8.0)]),
            (1, vec![(5, 13.5)]),
            (2, vec![(5, 16.5)]),
            (3, vec![(4, 12.0)]),
            (4, vec![(0, 8.0), (3, 12.0), (5, 2.0)]),
            (5, vec![(1, 13.5), (2, 16.5), (4, 2.0)]),
        ]);
        for key in graph.keys() {
            let g_vec = graph.get(key).unwrap();
            let a_vec = ans.get(key).unwrap();
            assert_eq!(g_vec, a_vec);
        }
        Ok(())
    }
}

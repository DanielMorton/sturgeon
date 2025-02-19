use crate::graph::tree::matrix::{find_minimal_coordinates, update_distance_matrix};
use crate::utils::{add_weighted_edge_pair, WeightedGraph};
use std::error::Error;

fn neighbor_joining_matrix(
    matrix: &mut [Vec<f64>],
    total_distance: &[f64],
) -> Result<(), Box<dyn Error>> {
    let matrix_len = matrix.len();
    let d = matrix_len as f64 - 2.0;

    for (i, row) in matrix.iter_mut().enumerate() {
        for j in 0..matrix_len {
            row[j] *= d;
            if i != j {
                row[j] -= total_distance[i];
                row[j] -= total_distance[j];
            }
        }
    }

    Ok(())
}

fn calculate_new_distances(d: &[Vec<f64>], i: usize, j: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut new_distances = Vec::new();

    for k in 0..d.len() {
        if k != i && k != j {
            let new_dist = (d[i][k] + d[j][k] - d[i][j]) / 2.0;
            new_distances.push(new_dist);
        }
    }

    // Add a placeholder for the distance to itself (which is 0)
    new_distances.push(0.0);
    Ok(new_distances)
}

pub fn neighbor_joining(matrix: &[Vec<f64>]) -> Result<WeightedGraph<usize, f64>, Box<dyn Error>> {
    let mut graph = WeightedGraph::new();
    let mut matrix_prime = matrix.to_vec();

    // Initialize clusters
    let mut clusters: Vec<usize> = (0..matrix.len()).collect();

    // Create a map to keep track of cluster sizes
    let mut cluster_sizes = vec![1; matrix.len()];

    while clusters.len() > 1 {
        let total_distance = matrix_prime
            .iter()
            .map(|row| row.iter().sum::<f64>())
            .collect::<Vec<_>>();
        let _ = neighbor_joining_matrix(&mut matrix_prime, &total_distance)?;
        let (i, j) = find_minimal_coordinates(&matrix_prime)?;
        let ci = clusters[i];
        let cj = clusters[j];

        let delta = (total_distance[i] - total_distance[j]) / (matrix_prime.len() as f64 - 2.0);
        let length_i = (matrix_prime[i][j] + delta) / 2.0;
        let length_j = (matrix_prime[i][j] - delta) / 2.0;
        let _ = add_weighted_edge_pair(&mut graph, cluster_sizes.len(), ci, length_i);
        let _ = add_weighted_edge_pair(&mut graph, cluster_sizes.len(), cj, length_j);

        let new_row = calculate_new_distances(&matrix_prime, i, j)?;
        let _ = update_distance_matrix(&mut matrix_prime, i, j, &new_row);
        clusters.remove(j);
        clusters.remove(i);
        clusters.push(cluster_sizes.len());
        let new_size = cluster_sizes[ci] + cluster_sizes[cj];
        cluster_sizes.push(new_size);
    }

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
            vec![20.0, 0.0, 30.0, 28.0],
            vec![27.0, 30.0, 0.0, 10.0],
            vec![20.0, 28.0, 30.0, 0.0],
        ];
        let graph = neighbor_joining(&matrix)?;
        let mut ans = HashMap::from([
            (0, vec![(4, 8.0)]),
            (1, vec![(5, 13.5)]),
            (2, vec![(5, 16.5)]),
            (3, vec![(4, 12.0)]),
            (4, vec![(5, 2.0), (0, 8.0), (3, 12.0)]),
            (5, vec![(1, 13.5), (2, 16.5), (4, 2.0)]),
        ]);
        for key in graph.keys() {
            let g_vec = graph.get(key).unwrap();
            let a_vec = graph.get(key).unwrap();
            for (&g, &a) in g_vec.iter().zip(a_vec.iter()) {
                assert_eq!(g.0, a.0);
                abs_diff_eq!(g.1, a.1);
            }
        }
        Ok(())
    }
}

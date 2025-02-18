use crate::graph::tree::matrix::update_distance_matrix;
use crate::graph::tree::merge::find_closest_clusters;
use crate::utils::{add_weighted_edge_pair, WeightedGraph};
use std::error::Error;

fn add_node(
    graph: &mut WeightedGraph<usize, f64>,
    ages: &[f64],
    new_node: usize,
    first: usize,
    second: usize,
) -> Result<(), Box<dyn Error>> {
    let _ = add_weighted_edge_pair(graph, new_node, first, ages[new_node] - ages[first]);
    let _ = add_weighted_edge_pair(graph, new_node, second, ages[new_node] - ages[second]);
    Ok(())
}

// UPGMA algorithm implementation
pub fn upgma(matrix: &[Vec<f64>]) -> Result<WeightedGraph<usize, f64>, Box<dyn Error>> {
    // Initialize the graph with n isolated nodes
    let mut graph = WeightedGraph::new();
    let mut matrix_prime = matrix.to_vec();

    // Initialize clusters
    let mut clusters: Vec<usize> = (0..matrix.len()).collect();

    // Create a map to keep track of cluster sizes
    let mut cluster_sizes = vec![1; matrix.len()];

    let mut ages = vec![0.0; matrix.len()];

    // Main algorithm loop
    while clusters.len() > 1 {
        // Find the two closest clusters
        let (i, j, min_dist) = find_closest_clusters(&matrix_prime)?;
        let ci = clusters[i];
        let cj = clusters[j];

        ages.push(min_dist / 2.0);
        // Create a new node in the graph
        let _ = add_node(&mut graph, &ages, cluster_sizes.len(), ci, cj);

        let new_size = cluster_sizes[ci] + cluster_sizes[cj];
        // Update the cluster sizes
        cluster_sizes.push(new_size);

        // Update the distance matrix
        let _ = update_distance_matrix(&mut matrix_prime, &mut clusters, i, j, &cluster_sizes);
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use crate::graph::tree::upgma::upgma;
    use approx::abs_diff_eq;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_upgma1() -> Result<(), Box<dyn Error>> {
        let matrix = vec![
            vec![0.0, 20.0, 17.0, 11.0],
            vec![20.0, 0.0, 20.0, 13.0],
            vec![17.0, 20.0, 0.0, 10.0],
            vec![11.0, 13.0, 10.0, 0.0],
        ];
        let graph = upgma(&matrix)?;
        let mut ans = HashMap::from([
            (0, vec![(5, 7.0)]),
            (1, vec![(6, 53.0 / 6.0)]),
            (2, vec![(4, 5.0)]),
            (3, vec![(4, 5.0)]),
            (4, vec![(2, 5.0), (3, 5.0), (5, 2.0)]),
            (5, vec![(0, 7.0), (4, 2.0), (6, 11.0 / 6.0)]),
            (6, vec![(1, 53.0 / 6.0), (5, 11.0 / 6.0)]),
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

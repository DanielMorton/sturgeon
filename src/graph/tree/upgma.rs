use crate::utils::{add_weighted_edge_pair, WeightedGraph};
use std::error::Error;
fn add_node(
    graph: &mut WeightedGraph<usize, f64>,
    ages: &[f64],
    new_node: usize,
    first: usize,
    second: usize,
) -> Result<(), Box<dyn Error>> {
    println!("id = {} first = {} second = {}", new_node, first, second);
    let _ = add_weighted_edge_pair(graph, new_node, first, ages[new_node] - ages[first]);
    let _ = add_weighted_edge_pair(graph, new_node, second, ages[new_node] - ages[second]);
    Ok(())
}

fn find_closest_clusters(d: &[Vec<f64>]) -> Result<(usize, usize, f64), Box<dyn Error>> {
    let mut min_i = 0;
    let mut min_j = 1;
    let mut min_dist = d[min_i][min_j];

    for i in 0..d.len() {
        for j in (i + 1)..d.len() {
            if d[i][j] < min_dist {
                min_i = i;
                min_j = j;
                min_dist = d[i][j];
            }
        }
    }

    Ok((min_i, min_j, min_dist))
}

fn update_distance_matrix(
    d: &mut Vec<Vec<f64>>,
    clusters: &mut Vec<usize>,
    i: usize,
    j: usize,
    cluster_sizes: &[usize],
) -> Result<(), Box<dyn Error>> {
    // Calculate new distances
    let new_row = calculate_new_distances(d, clusters, i, j, cluster_sizes)?;

    // Remove from larger index first to avoid shifting problems
    for row in d.iter_mut() {
        row.remove(j);
        row.remove(i);
    }
    d.remove(j);
    d.remove(i);

    // Add new row and column
    d.push(new_row.clone());
    let d_len = d.len();
    for (i, row) in d.iter_mut().enumerate().take(d_len - 1) {
        row.push(new_row[i]);
    }
    clusters.remove(j);
    clusters.remove(i);
    clusters.push(cluster_sizes.len() - 1);
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
        println!("Clusters {:?}", clusters);
        let ci = clusters[i];
        let cj = clusters[j];

        ages.push(min_dist / 2.0);
        // Create a new node in the graph
        let _ = add_node(&mut graph, &ages, cluster_sizes.len(), ci, cj);

        println!("{} {} {:?}", ci, cj, cluster_sizes);
        let new_size = cluster_sizes[ci] + cluster_sizes[cj];
        // Update the cluster sizes
        cluster_sizes.push(new_size);

        // Update the distance matrix
        let _ = update_distance_matrix(&mut matrix_prime, &mut clusters, i, j, &cluster_sizes);
    }

    Ok(graph)
}

// Helper function to calculate new distances after merging clusters
fn calculate_new_distances(
    d: &[Vec<f64>],
    clusters: &[usize],
    i: usize,
    j: usize,
    cluster_sizes: &[usize],
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut new_distances = Vec::new();
    let ci = clusters[i];
    let cj = clusters[j];

    for k in 0..clusters.len() {
        if k != i && k != j {
            let size_i = cluster_sizes[ci];
            let size_j = cluster_sizes[cj];

            // UPGMA formula for new distance
            let new_dist =
                (size_i as f64 * d[i][k] + size_j as f64 * d[j][k]) / ((size_i + size_j) as f64);

            new_distances.push(new_dist);
        }
    }

    // Add a placeholder for the distance to itself (which is 0)
    new_distances.push(0.0);
    println!("New Distances {:?}", new_distances);
    Ok(new_distances)
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

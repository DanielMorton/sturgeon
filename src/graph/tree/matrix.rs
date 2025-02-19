use std::error::Error;

pub(crate) fn find_minimal_coordinates(
    matrix: &[Vec<f64>],
) -> Result<(usize, usize), Box<dyn Error>> {
    if matrix.is_empty() {
        return Err("Empty matrix".into());
    }

    let matrix_size = matrix.len();
    let (mut min_i, mut min_j) = (0, 1);
    let mut min_value = f64::MAX;

    // Find minimum value, excluding diagonal elements
    for i in 0..matrix_size {
        for j in (i + 1)..matrix_size {
            if i != j {
                let value = matrix[i][j];
                if value < min_value {
                    min_i = i;
                    min_j = j;
                    min_value = value;
                }
            }
        }
    }

    Ok((min_i, min_j))
}

pub(crate) fn calculate_new_distances(
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
    Ok(new_distances)
}

pub(crate) fn update_distance_matrix(
    d: &mut Vec<Vec<f64>>,
    i: usize,
    j: usize,
    new_row: &[f64],
) -> Result<(), Box<dyn Error>> {
    // Calculate new distances

    // Remove from larger index first to avoid shifting problems
    for row in d.iter_mut() {
        row.remove(j);
        row.remove(i);
    }
    d.remove(j);
    d.remove(i);

    // Add new row and column
    d.push(new_row.to_vec());
    let d_len = d.len();
    for (i, row) in d.iter_mut().enumerate().take(d_len - 1) {
        row.push(new_row[i]);
    }
    Ok(())
}

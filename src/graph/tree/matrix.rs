use std::error::Error;
use std::iter::Sum;
use std::ops::Add;

pub(crate) fn row_sum<T>(matrix: &[Vec<T>]) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Add<Output = T> + Sum + Copy,
{
    Ok(matrix
        .iter()
        .map(|row| row.iter().copied().sum::<T>())
        .collect::<Vec<_>>())
}

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

pub(crate) fn new_upgma_row(
    d: &[Vec<f64>],
    clusters: &[usize],
    i: usize,
    j: usize,
    cluster_sizes: &[usize],
) -> Result<Vec<f64>, Box<dyn Error>> {
    let ci = clusters[i];
    let cj = clusters[j];
    let size_i = cluster_sizes[ci] as f64;
    let size_j = cluster_sizes[cj] as f64;

    Ok(d.into_iter()
        .enumerate()
        .filter(|(k, _)| k != &i && k != &j)
        .map(|(_, row)| (size_i * row[i] + size_j * row[j]) / (size_i + size_j))
        .chain([0.0])
        .collect::<Vec<_>>())
}

pub(crate) fn new_neigbhor_join_row(
    d: &[Vec<f64>],
    i: usize,
    j: usize,
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut new_row = Vec::new();

    for k in 0..d.len() {
        if k != i && k != j {
            let new_dist = (d[i][k] + d[j][k] - d[i][j]) / 2.0;
            new_row.push(new_dist);
        }
    }

    Ok(d.into_iter()
        .enumerate()
        .filter(|(k, _)| k != &i && k != &j)
        .map(|(_, row)| (row[i] + row[j] - d[i][j]) / 2.0)
        .chain([0.0])
        .collect::<Vec<_>>())
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

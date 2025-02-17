use crate::utils::WeightedGraph;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;

pub(crate) fn find_distance<T>(
    graph: &WeightedGraph<T, usize>,
    start: T,
    end: T,
) -> Result<usize, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, distance)) = queue.pop_front() {
        if current == end {
            return Ok(distance);
        }

        if let Some(neighbors) = graph.get(&current) {
            for &(next, weight) in neighbors {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, distance + weight));
                }
            }
        }
    }
    panic!("Graph not connected.")
}

fn compute_distances(
    graph: &WeightedGraph<usize, usize>,
) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut distances = vec![vec![0; graph.len()]; graph.len()];

    for &i in graph.keys() {
        for &j in graph.keys() {
            if i < j {
                distances[i][j] = find_distance(graph, i, j)?;
                distances[j][i] = distances[i][j]
            }
        }
    }

    Ok(distances)
}

pub fn calculate_limb_length(matrix: &[Vec<usize>], leaf: usize) -> Result<usize, Box<dyn Error>> {
    let mut min_length = usize::MAX;

    // Iterate through all pairs of leaves i and k
    for (i, row) in matrix.iter().enumerate() {
        for k in 0..matrix.len() {
            // Skip if i or k equals j, or if i equals k
            if i != leaf && k != leaf && i != k {
                // Calculate (Di,j + Dj,k - Di,k)/2
                let length = (row[leaf] + matrix[leaf][k] - row[k]) / 2;
                min_length = min_length.min(length);
            }
        }
    }

    Ok(min_length)
}

#[cfg(test)]
mod tests {
    use crate::graph::tree::distance::{calculate_limb_length, compute_distances};
    use std::collections::HashMap;
    use std::error::Error;

    fn test_compute_distances1() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![(4, 11)]),
            (1, vec![(4, 2)]),
            (2, vec![(5, 6)]),
            (3, vec![(5, 7)]),
            (4, vec![(0, 11), (1, 2), (5, 4)]),
            (5, vec![(4, 4), (3, 7), (2, 6)]),
        ]);
        let distances = compute_distances(&graph)?;
        assert_eq!(
            distances,
            vec![
                vec![0, 13, 21, 22],
                vec![13, 0, 12, 13],
                vec![21, 12, 0, 13],
                vec![22, 13, 13, 0]
            ]
        );
        Ok(())
    }

    fn test_calculate_limb_length1() -> Result<(), Box<dyn Error>> {
        let dist = vec![
            vec![0, 13, 21, 22],
            vec![13, 0, 12, 13],
            vec![21, 12, 0, 13],
            vec![22, 13, 13, 0],
        ];
        assert_eq!(calculate_limb_length(&dist, 1)?, 2);
        Ok(())
    }
}

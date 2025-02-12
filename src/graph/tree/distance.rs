use crate::utils::WeightedGraph;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;

fn find_distance<T>(
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
    panic!("Not a valid tree.")
}

fn compute_distances<T>(graph: &WeightedGraph<T, usize>) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut distances = vec![vec![0; graph.len()]; graph.len()];

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            if i < j {
                distances[i][j] = find_distance(graph, i, j)?;
                distances[j][i] = distances[i][j]
            }
        }
    }

    Ok(distances)
}

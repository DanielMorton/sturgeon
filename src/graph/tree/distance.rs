use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;
use crate::utils::WeightedGraph;

fn find_distance<T>(graph: WeightedGraph<T, usize>, start: T, end: T) -> Result<usize, Box<dyn Error>>
where T: Copy + Eq + Hash {
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
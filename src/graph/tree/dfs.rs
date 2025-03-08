use crate::utils::WeightedGraph;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

pub(crate) fn find_path<T>(
    graph: &WeightedGraph<T, usize>,
    start: T,
    target: T,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut stack = vec![(start, Vec::new())];

    while let Some((current, mut path)) = stack.pop() {
        path.push(current);

        if current == target {
            return Ok(path);
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if let Some(node) = graph.get(&current) {
            for &(next, _) in node.iter().rev() {
                // Reverse to maintain original DFS order
                if !visited.contains(&next) {
                    let next_path = path.clone();
                    stack.push((next, next_path));
                }
            }
        }
    }

    Ok(Vec::new())
}

pub(crate) fn find_node_at_distance<T>(
    graph: &WeightedGraph<T, usize>,
    start: T,
    target_distance: usize,
) -> Result<Option<T>, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut stack = vec![(start, 0)];

    while let Some((current, current_distance)) = stack.pop() {
        if current_distance == target_distance {
            return Ok(Some(current));
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if let Some(node) = graph.get(&current) {
            for &(next, weight) in node.iter().rev() {
                // Reverse to maintain original DFS order
                let new_distance = current_distance + weight;
                if !visited.contains(&next) && new_distance <= target_distance {
                    stack.push((next, new_distance));
                }
            }
        }
    }

    Ok(None)
}

pub(crate) fn find_edge_to_split<T>(
    graph: &WeightedGraph<T, usize>,
    path: &[T],
    target_distance: usize,
) -> Result<(T, T), Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut current_distance = 0;

    for window in path.windows(2) {
        let u = window[0];
        let v = window[1];
        let edge_weight = graph
            .get(&u)
            .unwrap()
            .iter()
            .find(|(node, _)| *node == v)
            .unwrap()
            .1;

        if current_distance + edge_weight >= target_distance {
            return Ok((u, v));
        }
        current_distance += edge_weight;
    }

    panic!("No suitable edge found");
}

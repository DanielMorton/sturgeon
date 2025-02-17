use crate::utils::WeightedGraph;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) fn find_path<T>(
    graph: &WeightedGraph<T, usize>,
    start: T,
    end: T,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut visited = std::collections::HashSet::new();
    let mut path = Vec::new();
    let _ = dfs_path(graph, start, end, &mut visited, &mut path);
    Ok(path)
}

fn dfs_path<T>(
    graph: &WeightedGraph<T, usize>,
    current: T,
    target: T,
    visited: &mut HashSet<T>,
    path: &mut Vec<T>,
) -> Result<bool, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    visited.insert(current);
    path.push(current);

    if current == target {
        return Ok(true);
    }

    if let Some(node) = graph.get(&current) {
        for &(next, _) in node {
            if !visited.contains(&next) {
                if dfs_path(graph, next, target, visited, path)? {
                    return Ok(true);
                }
            }
        }
    }

    path.pop();
    Ok(false)
}

pub(crate) fn find_node_at_distance<T>(
    graph: &WeightedGraph<T, usize>,
    start: T,
    target: T,
    distance: usize,
) -> Option<T>
where
    T: Copy + Eq + Debug + Hash,
{
    let mut visited = HashSet::new();
    println!("{:?}", graph);
    println!("{:?} {:?} {}", start, target, distance);
    dfs_find_node(graph, start, target, distance, 0, &mut visited)
}

fn dfs_find_node<T>(
    graph: &WeightedGraph<T, usize>,
    current: T,
    target: T,
    target_distance: usize,
    current_distance: usize,
    visited: &mut HashSet<T>,
) -> Option<T>
where
    T: Copy + Eq + Hash,
{
    if current_distance == target_distance {
        return Some(current);
    }

    visited.insert(current);

    if let Some(node) = graph.get(&current) {
        for &(next, weight) in node {
            if !visited.contains(&next) {
                let new_distance = current_distance + weight;
                if new_distance <= target_distance {
                    if let Some(result) =
                        dfs_find_node(graph, next, target, target_distance, new_distance, visited)
                    {
                        return Some(result);
                    }
                }
            }
        }
    }

    visited.remove(&current);
    None
}

pub(crate) fn find_edge_to_split<T: Copy>(
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

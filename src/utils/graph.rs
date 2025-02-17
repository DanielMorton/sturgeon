use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub type Graph<T> = HashMap<T, Vec<T>>;
pub type WeightedGraph<T, S> = HashMap<T, Vec<(T, S)>>;

pub fn add_weighted_edge<T, S>(
    graph: &mut WeightedGraph<T, S>,
    u: T,
    v: T,
    weight: S,
) -> Result<(), Box<dyn Error>>
where
    T: Hash + Eq,
{
    // Add edge in both directions since it's an undirected tree
    graph.entry(u).or_default().push((v, weight));
    Ok(())
}

pub fn add_weighted_edge_pair<T, S: Copy>(
    graph: &mut WeightedGraph<T, S>,
    u: T,
    v: T,
    weight: S,
) -> Result<(), Box<dyn Error>>
where
    T: Hash + Copy + Eq,
{
    let _ = add_weighted_edge(graph, u, v, weight);
    add_weighted_edge(graph, v, u, weight)
}

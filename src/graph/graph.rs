use std::error::Error;
use crate::utils::Graph;

pub type Contig<T> = Vec<T>;

pub(crate) fn reverse<T>(graph: &Graph<T>) -> Result<Graph<T>, Box<dyn Error>>
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut reverse_graph = Graph::new();
    for (key, values) in graph {
        for value in values {
            reverse_graph
                .entry(value.clone())
                .or_insert_with(Vec::new)
                .push(key.clone());
        }
    }
    Ok(reverse_graph)
}

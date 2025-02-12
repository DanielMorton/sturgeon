use crate::graph::err::EmptyGraphError;
use std::collections::HashMap;
use std::error::Error;
use crate::utils::Graph;

pub fn eulerian_path<T>(graph: &Graph<T>) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Clone + Eq + std::hash::Hash,
{
    if graph.is_empty() {
        return Err(Box::new(EmptyGraphError));
    }
    // Find the starting node (the one with a positive balance)
    let start = find_start(graph)?;

    // Perform the Eulerian path algorithm
    let mut stack = vec![start];
    let mut path = Vec::new();
    let mut graph_copy = graph.clone();

    while let Some(v) = stack.last().cloned() {
        if let Some(neighbors) = graph_copy.get_mut(&v) {
            if !neighbors.is_empty() {
                let next = neighbors.remove(0);
                stack.push(next);
            } else {
                if let Some(node) = stack.pop() {
                    path.push(node);
                }
            }
        }
    }

    // Reverse the answer to get the correct order
    path.reverse();
    Ok(path)
}

fn find_start<T>(graph: &Graph<T>) -> Result<T, Box<dyn Error>>
where
    T: Clone + Eq + std::hash::Hash,
{
    if graph.is_empty() {
        return Err(Box::new(EmptyGraphError));
    }

    let mut balance: HashMap<T, i32> = HashMap::new();
    for (node, neighbors) in graph.iter() {
        *balance.entry(node.clone()).or_insert(0) += neighbors.len() as i32;
        for neighbor in neighbors {
            *balance.entry(neighbor.clone()).or_insert(0) -= 1;
        }
    }

    Ok(balance
        .into_iter() // Convert to owned keys
        .find(|&(_, v)| v > 0)
        .map(|(k, _)| k)
        .unwrap_or_else(|| graph.keys().next().unwrap().clone()))
}
#[cfg(test)]
mod tests {
    use crate::graph::euler::path::eulerian_path;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_eulerian_path1() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![2]),
            (1, vec![3]),
            (2, vec![1]),
            (3, vec![0, 4]),
            (4, vec![]),
            (6, vec![3, 7]),
            (7, vec![8]),
            (8, vec![9]),
            (9, vec![6]),
        ]);
        let path = eulerian_path::<i32>(&graph)?;
        assert_eq!(path, vec![6, 7, 8, 9, 6, 3, 0, 2, 1, 3, 4]);
        Ok(())
    }

    #[test]
    fn test_eulerian_path2() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([(0, vec![1]), (1, vec![2]), (2, vec![3]), (3, vec![])]);
        let path = eulerian_path::<i32>(&graph)?;
        assert_eq!(path, vec![0, 1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_eulerian_path3() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![1]),
            (1, vec![2, 5]),
            (2, vec![3]),
            (3, vec![4]),
            (4, vec![1]),
            (5, vec![]),
        ]);
        let path = eulerian_path::<i32>(&graph)?;
        assert_eq!(path, vec![0, 1, 2, 3, 4, 1, 5]);
        Ok(())
    }

    #[test]
    fn test_eulerian_path4() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![]),
            (2, vec![1]),
            (1, vec![3, 4, 0]),
            (3, vec![1, 4]),
            (4, vec![3, 1]),
        ]);
        let path = eulerian_path::<i32>(&graph)?;
        assert_eq!(path, vec![2, 1, 3, 1, 4, 3, 4, 1, 0]);
        Ok(())
    }

    #[test]
    fn test_eulerian_path5() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![1]),
            (1, vec![14, 17]),
            (14, vec![2, 3, 4]),
            (2, vec![1]),
            (3, vec![14]),
            (4, vec![5]),
            (5, vec![14]),
            (17, vec![]),
        ]);
        let path = eulerian_path::<i32>(&graph)?;
        assert_eq!(path, vec![0, 1, 14, 3, 14, 4, 5, 14, 2, 1, 17]);
        Ok(())
    }
}

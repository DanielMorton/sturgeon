use std::collections::{HashMap, HashSet};
use std::error::Error;

/// Creates a graph representation from two sets of integer blocks
pub fn make_graph(
    p: &[Vec<i32>],
    q: &[Vec<i32>],
) -> Result<HashMap<i32, HashSet<i32>>, Box<dyn Error>> {
    let mut edges = HashMap::new();

    // Initialize edges
    for block in p.iter().chain(q.iter()) {
        let b = block.len();
        for (i, &value) in block.iter().enumerate() {
            edges.entry(value).or_insert_with(HashSet::new);
            let next_val = -block[(i + 1) % b];
            edges.entry(next_val).or_insert_with(HashSet::new);
        }
    }

    // Add connections
    for block in p.iter().chain(q.iter()) {
        let b = block.len();
        for (i, &value) in block.iter().enumerate() {
            let next_val = -block[(i + 1) % b];
            edges.get_mut(&value).unwrap().insert(next_val);
            edges.get_mut(&next_val).unwrap().insert(value);
        }
    }

    Ok(edges)
}

/// Calculates the two-break distance between two arrangements
pub fn two_break_distance(p: &[Vec<i32>], q: &[Vec<i32>]) -> Result<usize, Box<dyn Error>> {
    let blocks = p.iter().map(|block| block.len()).sum::<usize>();

    let mut edges = make_graph(p, q)?;
    let mut cycles = 0;

    while !edges.is_empty() {
        cycles += 1;

        // Find first available node
        let start_node = *edges.keys().next().unwrap();
        let mut node = start_node;

        while let Some(edge_set) = edges.get(&node) {
            // Get next node in cycle
            let next = *edge_set.iter().next().unwrap();

            // Remove edges
            if let Some(node_edges) = edges.get_mut(&node) {
                node_edges.remove(&next);
                if node_edges.is_empty() {
                    edges.remove(&node);
                }
            }

            if let Some(next_edges) = edges.get_mut(&next) {
                next_edges.remove(&node);
                if next_edges.is_empty() {
                    edges.remove(&next);
                }
            }

            node = next;
        }
    }

    Ok(blocks - cycles)
}

/// Perform two-break operation on graph
pub(crate) fn two_break_on_graph(
    edges: &mut Vec<(i32, i32)>,
    breaks: &[(i32, i32)],
) -> Result<(), Box<dyn Error>> {
    let (i0, i1, j0, j1) = (breaks[0].0, breaks[0].1, breaks[1].0, breaks[1].1);
    let to_remove = HashSet::from([(i0, i1), (i1, i0), (j0, j1), (j1, j0)]);
    edges.retain(|x| !to_remove.contains(x));

    edges.push((i0, j0));
    edges.push((i1, j1));
    edges.sort();
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::genome::two_break::two_break_distance;
    use std::error::Error;

    #[test]
    fn test_two_break_distance1() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5, 6]];
        let q = vec![vec![1, -3, -6, -5], vec![2, -4]];

        assert_eq!(two_break_distance(&p, &q)?, 3);
        Ok(())
    }

    #[test]
    fn test_two_break_distance2() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];
        let q = vec![vec![1, 2, 3, 4, 5], vec![6, 7]];

        assert_eq!(two_break_distance(&p, &q)?, 0);
        Ok(())
    }

    #[test]
    fn test_two_break_distance3() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, -2, -3, 4]];
        let q = vec![vec![1, 2, -4, -3]];

        assert_eq!(two_break_distance(&p, &q)?, 3);
        Ok(())
    }

    #[test]
    fn test_two_break_distance4() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, -2, -3, 4]];
        let q = vec![vec![1, 2, -4, -3]];

        assert_eq!(two_break_distance(&p, &q)?, 3);
        Ok(())
    }

    #[test]
    fn test_two_break_distance5() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3], vec![-4, -5]];
        let q = vec![vec![-1, -2, -3], vec![4, 5]];

        assert_eq!(two_break_distance(&p, &q)?, 2);
        Ok(())
    }

    #[test]
    fn test_two_break_distance6() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![-1, 2, 3, 4, 5, -6]];
        let q = vec![vec![1, 2, 3, 4, 5, 6]];

        assert_eq!(two_break_distance(&p, &q)?, 2);
        Ok(())
    }

    #[test]
    fn test_two_break_distance7() -> Result<(), Box<dyn Error>> {
        let p = vec![vec![1, 2, 3], vec![4, 5]];
        let q = vec![vec![2, -1, 4, -3, 5]];

        assert_eq!(two_break_distance(&p, &q)?, 4);
        Ok(())
    }
}

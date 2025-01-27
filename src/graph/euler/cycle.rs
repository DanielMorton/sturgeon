use std::collections::HashMap;
use std::error::Error;
use crate::graph::err::EmptyGraphError;

pub fn eulerian_cycle<T>(graph: &HashMap<T, Vec<T>>) -> Result<Vec<T>, Box<dyn Error>>
    where T: Clone + Eq + std::hash::Hash {
    let mut stack = Vec::new();
    let mut cycle = Vec::new();
    let mut graph_copy = graph.clone();

    // Get first vertex
    let first = graph.keys().next().cloned().ok_or(EmptyGraphError)?;
    stack.push(first);

    while let Some(v) = stack.last().cloned() {
        if let Some(neighbors) = graph_copy.get_mut(&v) {
            if !neighbors.is_empty() {
                let next = neighbors.remove(0);
                stack.push(next);
            } else {
                if let Some(node) = stack.pop() {
                    cycle.push(node);
                }
            }
        }
    }

    cycle.reverse();
    Ok(cycle)
}

mod tests {
    use std::collections::HashMap;
    use std::error::Error;
    use crate::graph::euler::cycle::eulerian_cycle;

    #[test]
    fn test_eulerian_cycle1() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![3]),
            (1, vec![0]),
            (2, vec![1, 6]),
            (3, vec![2]),
            (4, vec![2]),
            (5, vec![4]),
            (6, vec![5, 8]),
            (7, vec![9]),
            (8, vec![7]),
            (9, vec![6]),
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 0).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(0);
        assert_eq!(cycle, vec![0, 3, 2, 6, 8, 7, 9, 6, 5, 4, 2, 1, 0]);
        Ok(())
    }

    #[test]
    fn test_eulerian_cycle2() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![1]),
            (1, vec![2]),
            (2, vec![0])
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 0).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(0);
        assert_eq!(cycle, vec![0, 1, 2, 0]);
        Ok(())
    }

    #[test]
    fn test_eulerian_cycle3() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![3, 1]),
            (1, vec![2]),
            (2, vec![0]),
            (3, vec![0])
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 3).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(3);
        assert_eq!(cycle, vec![3, 0, 1, 2, 0, 3]);
        Ok(())
    }

    #[test]
    fn test_eulerian_cycle4() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (0, vec![1]),
            (1, vec![2, 3]),
            (2, vec![0]),
            (3, vec![4]),
            (4, vec![1])
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 0).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(0);
        assert_eq!(cycle, vec![0, 1, 3, 4, 1, 2, 0]);
        Ok(())
    }

    #[test]
    fn test_eulerian_cycle5() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (1, vec![2]),
            (2, vec![1, 2])
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 1).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(1);
        assert_eq!(cycle, vec![1, 2, 2, 1]);
        Ok(())
    }

    #[test]
    fn test_eulerian_cycle6() -> Result<(), Box<dyn Error>> {
        let graph = HashMap::from([
            (1, vec![10]),
            (10, vec![2, 3, 4]),
            (2, vec![1]),
            (3, vec![10]),
            (4, vec![5]),
            (5, vec![10])
        ]);
        let mut cycle = eulerian_cycle::<i32>(&graph)?;
        cycle = cycle[..cycle.len() - 1].to_owned();
        let first_zero_pos = cycle.iter().position(|&x| x == 1).unwrap();
        cycle.rotate_left(first_zero_pos);
        cycle.push(1);
        assert_eq!(cycle, vec![1, 10, 3, 10, 4, 5, 10, 2, 1]);
        Ok(())
    }
}
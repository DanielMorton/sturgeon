use crate::utils::{find_parent, union};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub(crate) fn group_nodes<T>(edges: &[(T, T)]) -> Result<HashMap<T, T>, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let mut parent = HashMap::new();
    let mut rank = HashMap::new();

    // Initialize parent and rank in a single pass
    for &(a, b) in edges {
        for &node in &[a, b] {
            parent.entry(node).or_insert(node);
            rank.entry(node).or_insert(0);
        }
    }

    // Process unions
    for &(a, b) in edges {
        // Connect nodes in edge
        union(a, b, &mut parent, &mut rank)?;
    }

    let mut node_parents = HashMap::new();

    let parent_keys = parent.keys().copied().collect::<Vec<_>>();

    for node in parent_keys {
        let id = find_parent(node, &mut parent)?;
        node_parents.insert(node, id);
    }

    Ok(node_parents)
}

pub(crate) fn group_nodes_pairs(edges: &[(i32, i32)]) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut parent = HashMap::new();
    let mut rank = HashMap::new();

    // Initialize parent and rank in a single pass
    for &(a, b) in edges {
        for &node in &[a, b] {
            parent.entry(node).or_insert(node);
            rank.entry(node).or_insert(0);
        }
    }

    // Process unions
    for &(a, b) in edges {
        // Connect nodes in edge
        union(a, b, &mut parent, &mut rank)?;

        // Connect odd/even pairs
        let a_pair = a + (a % 2 * 2 - 1);
        let b_pair = b + (b % 2 * 2 - 1);
        union(a, a_pair, &mut parent, &mut rank)?;
        union(b, b_pair, &mut parent, &mut rank)?;
    }

    let mut node_parents = HashMap::new();

    let parent_keys = parent.keys().copied().collect::<Vec<_>>();

    for node in parent_keys {
        let id = find_parent(node, &mut parent)?;
        node_parents.insert(node, id);
    }

    Ok(node_parents)
}

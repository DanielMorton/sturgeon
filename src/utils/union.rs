use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub fn find_parent<T>(i: T, parent: &mut HashMap<T, T>) -> Result<T, Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    let p = *parent.get(&i).unwrap();
    Ok(if i != p {
        let new_parent = find_parent(p, parent)?;
        parent.insert(i, new_parent);
        new_parent
    } else {
        i
    })
}

// Union by rank
pub fn union<T>(
    mut i: T,
    mut j: T,
    parent: &mut HashMap<T, T>,
    rank: &mut HashMap<T, usize>,
) -> Result<(), Box<dyn Error>>
where
    T: Copy + Eq + Hash,
{
    i = find_parent(i, parent)?;
    j = find_parent(j, parent)?;

    if i != j {
        let i_rank = *rank.get(&i).unwrap();
        let j_rank = *rank.get(&j).unwrap();

        if i_rank > j_rank {
            parent.insert(j, i);
        } else {
            parent.insert(i, j);
            if i_rank == j_rank {
                rank.entry(j).and_modify(|r| *r += 1);
            }
        }
    }
    Ok(())
}

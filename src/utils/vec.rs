use std::collections::HashMap;
use std::error::Error;

pub fn vec_to_count<T>(vec: &[T]) -> Result<HashMap<T, usize>, Box<dyn Error>>
    where
        T: Clone + Eq + std::hash::Hash{
    let mut count = HashMap::new();
    for v in vec {
        *count.entry(v.clone()).or_insert(0) +=1
    }
    Ok(count)
}
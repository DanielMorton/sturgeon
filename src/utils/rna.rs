use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static RNA: Lazy<Vec<char>> = Lazy::new(|| vec!['A', 'C', 'G', 'U']);

pub static RNA_COMPLEMENT_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('A', 'U');
    m.insert('U', 'A');
    m.insert('C', 'G');
    m.insert('G', 'C');
    m
});
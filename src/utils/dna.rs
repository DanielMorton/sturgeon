use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static DNA: Lazy<Vec<char>> = Lazy::new(|| vec!['A', 'C', 'G', 'T']);

pub static DNA_COMPLEMENT_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('A', 'T');
    m.insert('T', 'A');
    m.insert('C', 'G');
    m.insert('G', 'C');
    m
});

pub static DNA_INDEX: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('A', 0);
    m.insert('C', 1);
    m.insert('G', 2);
    m.insert('T', 3);
    m
});

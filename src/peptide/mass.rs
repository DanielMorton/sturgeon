use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub(crate) fn read_masses() -> Result<HashMap<char, usize>, Box<dyn Error>> {
    Ok(fs::read_to_string("integer_mass_table.txt")?
        .split('\n')
        .map(|mass| mass.split(' ').collect::<Vec<_>>())
        .map(|mass| (mass[0].chars().next().unwrap(), mass[1].parse::<usize>().unwrap()))
        .collect::<HashMap<_, _>>())
}
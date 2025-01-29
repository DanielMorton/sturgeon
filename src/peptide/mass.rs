use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub(crate) fn read_masses() -> Result<HashMap<char, usize>, Box<dyn Error>> {
    Ok(fs::read_to_string("integer_mass_table.txt")?
        .split('\n')
        .map(|mass| mass.split(' ').collect::<Vec<_>>())
        .map(|mass| {
            (
                mass[0].chars().next().unwrap(),
                mass[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>())
}

pub(crate) fn make_mass_vector() -> Result<Vec<usize>, Box<dyn Error>> {
    let mut masses = fs::read_to_string("integer_mass_table.txt")?
        .split('\n')
        .map(|mass| mass.split(' ').collect::<Vec<_>>())
        .map(|mass| mass[1].parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    masses.sort();
    masses.dedup();
    Ok(masses)
}

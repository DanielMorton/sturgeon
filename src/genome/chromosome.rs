use num::Num;
use std::error::Error;
use std::ops::Neg;

pub(crate) fn chromosome_to_cycle(chromosome: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok(chromosome
        .iter()
        .map(|&c| {
            if c > 0 {
                vec![2 * c - 1, 2 * c]
            } else {
                vec![-2 * c, -2 * c - 1]
            }
        })
        .flat_map(|v| v.into_iter())
        .collect::<Vec<_>>())
}

/// Convert cycle back to chromosome representation
pub(crate) fn cycle_to_chromosome(nodes: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok((0..nodes.len() / 2)
        .map(|j| {
            if nodes[2 * j] < nodes[2 * j + 1] {
                nodes[2 * j + 1] / 2
            } else {
                -nodes[2 * j] / 2
            }
        })
        .collect::<Vec<_>>())
}

/// Generate colored edges for a genome
pub(crate) fn colored_edges(genome: &[Vec<i32>]) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let mut edges = Vec::new();

    for chromosome in genome {
        let mut nodes = chromosome_to_cycle(chromosome)?;
        nodes.push(nodes[0]);

        for j in 0..chromosome.len() {
            edges.push((nodes[2 * j + 1], nodes[2 * j + 2]));
        }
    }
    edges.sort();

    Ok(edges)
}

fn reverse_chromosome<T>(chromosome: &[T]) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Copy + Neg<Output = T>,
{
    let mut reverse_chromosome = chromosome.iter().map(|&x| -x).collect::<Vec<_>>();
    reverse_chromosome.reverse();
    Ok(reverse_chromosome)
}

fn sort_chromosomme<T>(chromosome: &[T]) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Copy + PartialEq + Neg<Output = T> + Num + Ord,
{
    let min_gene = chromosome
        .iter()
        .map(|&x| if x > T::zero() { x } else { -x })
        .min()
        .unwrap();
    if chromosome.contains(&min_gene) {
        let pos = chromosome.iter().position(|&x| x == min_gene).unwrap();
        let mut sorted = chromosome[pos..].to_owned();
        sorted.append(&mut chromosome[..pos].to_owned());
        Ok(sorted)
    } else {
        let reverse = reverse_chromosome(chromosome)?;
        sort_chromosomme(&reverse)
    }
}

pub(crate) fn sort_genome(genome: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut sorted = genome
        .iter()
        .map(|c| sort_chromosomme(c))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    sorted.sort();
    Ok(sorted)
}

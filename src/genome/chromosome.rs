use std::collections::HashSet;
use std::error::Error;

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
pub(crate) fn colored_edges(genome: &[Vec<i32>]) -> Result<HashSet<(i32, i32)>, Box<dyn Error>> {
    let mut edges = HashSet::new();

    for chromosome in genome {
        let mut nodes = chromosome_to_cycle(chromosome)?;
        println!("{:?}", nodes);
        nodes.push(nodes[0]);

        for j in 0..chromosome.len() {
            edges.insert((nodes[2 * j + 1], nodes[2 * j + 2]));
        }
    }

    Ok(edges)
}

fn reverse_chromosome(chromosome: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut reverse_chromosome = chromosome.iter().map(|&x| -x).collect::<Vec<_>>();
    reverse_chromosome.reverse();
    Ok(reverse_chromosome)
}

fn sort_chromosomme(chromosome: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    let min_gene = chromosome.iter().map(|x| x.abs()).min().unwrap();
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
    genome
        .iter()
        .map(|c| sort_chromosomme(c))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
}

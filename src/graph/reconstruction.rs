use crate::graph::debruijn::debruijn_kmers;
use crate::graph::err::{EmptyPathError, InvalidPathError};
use crate::graph::euler::eulerian_path;
use std::error::Error;

fn string_reconstruction(patterns: &[String]) -> Result<String, Box<dyn Error>> {
    let graph = debruijn_kmers(patterns)?;
    let path = eulerian_path(&graph)?;
    let genome = genome_path(&path)?;
    Ok(genome)
}

pub fn genome_path(path: &[String]) -> Result<String, Box<dyn Error>> {
    if path.is_empty() {
        return Err(Box::new(EmptyPathError));
    }

    let mut genome = path[0].to_owned();

    for i in 1..path.len() {
        // Check if the prefix of path[i] matches the suffix of path[i-1]
        if !path[i].starts_with(&path[i - 1][1..]) {
            return Err(Box::new(InvalidPathError::new(i, &path[i - 1], &path[i])));
        }

        // Add the last character of path[i] to the result
        if let Some(last_char) = path[i].chars().last() {
            genome.push(last_char);
        }
    }
    Ok(genome)
}

fn genome_pair_path(path: &[String], gap: usize) -> Result<String, Box<dyn Error>> {
    let path_split = path
        .iter()
        .map(|s| s.split('|').collect::<Vec<_>>())
        .map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut head = path_split[0][0].clone();
    let mut tail = path_split[0][1].clone();
    let k = head.len();

    for p in &path_split[1..] {
        head.push_str(&p[0][k - 1..k]);
        tail.push_str(&p[1][k - 1..k]);
    }

    if head[k + gap..] != tail[..tail.len() - k - gap] {
        let mut new_path = path[1..].to_vec();
        new_path.push(path[0].clone());
        return genome_pair_path(&new_path, gap);
    }

    let mut result = head;
    result.push_str(&tail[tail.len() - k - gap..]);
    Ok(result)
}

mod tests {
    use crate::graph::reconstruction::{genome_path, string_reconstruction};
    use std::error::Error;

    #[test]
    fn test_genome_path1() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("ACCGA"),
            format!("CCGAA"),
            format!("CGAAG"),
            format!("GAAGC"),
            format!("AAGCT"),
        ];
        assert_eq!(genome_path(&path)?, "ACCGAAGCT");
        Ok(())
    }

    #[test]
    fn test_genome_path2() -> Result<(), Box<dyn Error>> {
        let path = vec![format!("CTT"), format!("TTT"), format!("TTG")];
        assert_eq!(genome_path(&path)?, "CTTTG");
        Ok(())
    }

    #[test]
    fn test_genome_path3() -> Result<(), Box<dyn Error>> {
        let path = vec![format!("TT"), format!("TG"), format!("GT"), format!("TT")];
        assert_eq!(genome_path(&path)?, "TTGTT");
        Ok(())
    }

    #[test]
    fn test_genome_path4() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("AGCAGATC"),
            format!("GCAGATCA"),
            format!("CAGATCAT"),
            format!("AGATCATC"),
            format!("GATCATCG"),
            format!("ATCATCGG"),
        ];
        assert_eq!(genome_path(&path)?, "AGCAGATCATCGG");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction1() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("ACG"),
            format!("CGT"),
            format!("GTG"),
            format!("TGT"),
            format!("GTA"),
            format!("TAT"),
            format!("ATA"),
        ];
        assert_eq!(string_reconstruction(&path)?, "ACGTGTATA");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction2() -> Result<(), Box<dyn Error>> {
        let path = vec![format!("GG"), format!("AC"), format!("GA"), format!("CT")];
        assert_eq!(string_reconstruction(&path)?, "GGACT");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction3() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("AAC"),
            format!("AAC"),
            format!("ACG"),
            format!("ACT"),
            format!("CGA"),
            format!("GAA"),
        ];
        assert_eq!(string_reconstruction(&path)?, "AACGAACT");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction4() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("CTAC"),
            format!("CTCC"),
            format!("TCCT"),
            format!("ACTC"),
            format!("CCTC"),
            format!("CCTA"),
            format!("TACT"),
        ];
        assert_eq!(string_reconstruction(&path)?, "CCTACTCCTC");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction5() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("CCC"),
            format!("CCC"),
            format!("CCC"),
            format!("TCC"),
            format!("CCC"),
            format!("CCG"),
            format!("CCC"),
            format!("CCC"),
            format!("CCC"),
        ];
        assert_eq!(string_reconstruction(&path)?, "TCCCCCCCCCG");
        Ok(())
    }

    #[test]
    fn test_string_reconstruction7() -> Result<(), Box<dyn Error>> {
        let path = vec![
            format!("ACG"),
            format!("CGT"),
            format!("GTA"),
            format!("TAC"),
        ];
        assert_eq!(string_reconstruction(&path)?, "ACGTAC");
        Ok(())
    }
}

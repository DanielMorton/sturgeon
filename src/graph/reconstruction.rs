use crate::graph::err::{EmptyPathError, InvalidPathError};
use std::error::Error;

fn genome_path(path: &[String]) -> Result<String, Box<dyn Error>> {
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

mod tests {
    use crate::graph::reconstruction::genome_path;
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
}

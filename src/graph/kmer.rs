use std::error::Error;
use std::ops::Deref;

fn kmer_composition(text: &str, k: usize) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(text.as_bytes().windows(k).map(String::from_utf8_lossy)
        .map(|s|s.deref().to_owned()).collect::<Vec<_>>())
}

mod tests {
    use std::error::Error;
    use crate::graph::kmer::kmer_composition;

    #[test]
    fn kmer_composition1() -> Result<(), Box<dyn Error>> {
        let text = "CAATCCAAC";
        assert_eq!(
            kmer_composition(&text, 5)?,
            vec!["CAATC", "AATCC", "ATCCA", "TCCAA", "CCAAC"]
        );
        Ok(())
    }

    #[test]
    fn kmer_composition2() -> Result<(), Box<dyn Error>> {
        let text = "TCGAA";
        assert_eq!(
            kmer_composition(&text, 3)?,
            vec!["TCG", "CGA", "GAA"]
        );
        Ok(())
    }

    #[test]
    fn kmer_composition3() -> Result<(), Box<dyn Error>> {
        let text = "CCCCCCC";
        assert_eq!(
            kmer_composition(&text, 2)?,
            vec!["CC", "CC", "CC", "CC", "CC", "CC"]
        );
        Ok(())
    }

    #[test]
    fn kmer_composition4() -> Result<(), Box<dyn Error>> {
        let text = "ACGT";
        assert_eq!(
            kmer_composition(&text, 4)?,
            vec!["ACGT"]
        );
        Ok(())
    }

    #[test]
    fn kmer_composition5() -> Result<(), Box<dyn Error>> {
        let text = "GGGGGGTGGG";
        assert_eq!(
            kmer_composition(&text, 3)?,
            vec!["GGG", "GGG", "GGG", "GGG", "GGT", "GTG", "TGG", "GGG"]
        );
        Ok(())
    }
}
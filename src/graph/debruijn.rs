use std::collections::HashMap;
use std::error::Error;

pub fn debruijn_string(
    text: &str,
    kmer_length: usize,
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut graph = HashMap::new();

    for i in 0..=text.len() - kmer_length {
        let pattern = &text[i..i+kmer_length];
        let prefix = pattern[..kmer_length-1].to_owned();
        let suffix = pattern[1..].to_owned();

        graph.entry(prefix)
            .or_insert_with(Vec::new)
            .push(suffix);
    }

    for value in graph.values_mut() {
        value.sort();
    }

    Ok(graph)
}

pub fn debruijn_kmers(patterns: &[String]) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let p = patterns[0].len();
    let mut graph = HashMap::new();

    for pattern in patterns {
        graph.entry(pattern[..p-1].to_owned())
            .or_insert_with(Vec::new)
            .push(pattern[1..].to_owned());
    }

    for value in graph.values_mut() {
        value.sort();
    }

    Ok(graph)
}

mod tests {
    use crate::graph::debruijn::{debruijn_kmers, debruijn_string};
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_debruijn_string1() -> Result<(), Box<dyn Error>> {
        let text = format!("ACGTGTATA");
        let ans = HashMap::from([
            (format!("AC"), vec![format!("CG")]),
            (format!("AT"), vec![format!("TA")]),
            (format!("CG"), vec![format!("GT")]),
            (format!("GT"), vec![format!("TA"), format!("TG")]),
            (format!("TA"), vec![format!("AT")]),
            (format!("TG"), vec![format!("GT")]),
        ]);
        assert_eq!(debruijn_string(&text, 3)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_string2() -> Result<(), Box<dyn Error>> {
        let text = format!("AGCCT");
        let ans = HashMap::from([
            (format!("AGC"), vec![format!("GCC")]),
            (format!("GCC"), vec![format!("CCT")])
        ]);
        assert_eq!(debruijn_string(&text, 4)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_string3() -> Result<(), Box<dyn Error>> {
        let text = format!("CCTCCG");
        let ans = HashMap::from([
            (format!("CC"), vec![format!("CG"), format!("CT")]),
            (format!("CT"), vec![format!("TC")]),
            (format!("TC"), vec![format!("CC")])
        ]);
        assert_eq!(debruijn_string(&text, 3)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_string4() -> Result<(), Box<dyn Error>> {
        let text = format!("GCTTCTTC");
        let ans = HashMap::from([
            (format!("CTT"), vec![format!("TTC"), format!("TTC")]),
            (format!("GCT"), vec![format!("CTT")]),
            (format!("TCT"), vec![format!("CTT")]),
            (format!("TTC"), vec![format!("TCT")])
        ]);
        assert_eq!(debruijn_string(&text, 4)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_string5() -> Result<(), Box<dyn Error>> {
        let text = format!("TTTTTTTTTT");
        let ans = HashMap::from([
            (format!("TTTT"), vec![format!("TTTT"), format!("TTTT"),
                                   format!("TTTT"), format!("TTTT"),
                                   format!("TTTT"), format!("TTTT")])
        ]);
        assert_eq!(debruijn_string(&text, 5)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_kmers1() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("GAGG"),
            format!("CAGG"),
            format!("GGGG"),
            format!("GGGA"),
            format!("CAGG"),
            format!("AGGG"),
            format!("GGAG")
        ];
        let ans = HashMap::from([
            (format!("AGG"), vec![format!("GGG")]),
            (format!("CAG"), vec![format!("AGG"), format!("AGG")]),
            (format!("GAG"), vec![format!("AGG")]),
            (format!("GGA"), vec![format!("GAG")]),
            (format!("GGG"), vec![format!("GGA"), format!("GGG")])
        ]);
        assert_eq!(debruijn_kmers(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_kmers2() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("GCAAG"),
            format!("CAGCT"),
            format!("TGACG")
        ];
        let ans = HashMap::from([
            (format!("CAGC"), vec![format!("AGCT")]),
            (format!("GCAA"), vec![format!("CAAG")]),
            (format!("TGAC"), vec![format!("GACG")])
        ]);
        assert_eq!(debruijn_kmers(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_kmers3() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("AGGT"),
            format!("GGCT"),
            format!("AGGC")
        ];
        let ans = HashMap::from([
            (format!("AGG"), vec![format!("GGC"), format!("GGT")]),
            (format!("GGC"), vec![format!("GCT")])
        ]);
        assert_eq!(debruijn_kmers(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_kmers4() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("TTCT"),
            format!("GGCT"),
            format!("AAGT"),
            format!("GGCT"),
            format!("TTCT")
        ];
        let ans = HashMap::from([
            (format!("AAG"), vec![format!("AGT")]),
            (format!("GGC"), vec![format!("GCT"), format!("GCT")]),
            (format!("TTC"), vec![format!("TCT"), format!("TCT")])
        ]);
        assert_eq!(debruijn_kmers(&patterns)?, ans);
        Ok(())
    }

    #[test]
    fn test_debruijn_kmers5() -> Result<(), Box<dyn Error>> {
        let patterns = vec![
            format!("CA"),
            format!("CA"),
            format!("CA"),
            format!("CA"),
            format!("CC"),
            format!("CA")
        ];
        let ans = HashMap::from([
            (format!("C"), vec![format!("A"), format!("A"), format!("A"), format!("A"), format!("A"), format!("C")])
        ]);
        assert_eq!(debruijn_kmers(&patterns)?, ans);
        Ok(())
    }
}

use crate::motif::neighbors::neighbors;
use crate::utils::dna_complement;
use std::collections::HashMap;
use std::error::Error;

fn make_pattern_freq(
    text: &str,
    k: usize,
    max_diff: usize,
    char_set: &[char],
    pattern_freq: &mut HashMap<String, usize>,
) -> Result<(), Box<dyn Error>> {
    for i in 0..=(text.len() - k) {
        for n in neighbors(&text[i..i + k], max_diff, char_set)? {
            *pattern_freq.entry(n).or_insert(0) += 1;
        }
    }
    Ok(())
}

pub fn frequent_words_with_mismatches(
    text: &str,
    k: usize,
    max_diff: usize,
    char_set: &[char],
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut pattern_freq: HashMap<String, usize> = HashMap::new();
    make_pattern_freq(text, k, max_diff, char_set, &mut pattern_freq)?;
    let max_freq = pattern_freq.values().max().copied().unwrap_or(0);

    Ok(pattern_freq
        .into_iter()
        .filter(|&(_, freq)| freq == max_freq)
        .map(|(pattern, _)| pattern)
        .collect())
}

pub fn frequent_words_with_mismatches_reverse_complement(
    text: &str,
    k: usize,
    max_diff: usize,
    char_set: &[char],
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut pattern_freq: HashMap<String, usize> = HashMap::new();
    make_pattern_freq(text, k, max_diff, char_set, &mut pattern_freq)?;
    let reverse_text = dna_complement(text)?;
    make_pattern_freq(&reverse_text, k, max_diff, char_set, &mut pattern_freq)?;
    let max_freq = pattern_freq.values().max().copied().unwrap_or(0);

    Ok(pattern_freq
        .into_iter()
        .filter(|&(_, freq)| freq == max_freq)
        .map(|(pattern, _)| pattern)
        .collect())
}

mod test {
    use crate::motif::mismatch::{
        frequent_words_with_mismatches, frequent_words_with_mismatches_reverse_complement,
    };
    use crate::utils::DNA;
    use std::error::Error;

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement1() -> Result<(), Box<dyn Error>> {
        let text = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 4, 1, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["ACAT", "ATGT"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement2() -> Result<(), Box<dyn Error>> {
        let text = "AAAAAAAAAA";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 2, 1, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AT", "TA"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement3() -> Result<(), Box<dyn Error>> {
        let text = "AGTCAGTC";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 4, 2, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AATT", "GGCC"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement4() -> Result<(), Box<dyn Error>> {
        let text = "AATTAATTGGTAGGTAGGTA";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 4, 0, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AATT"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement5() -> Result<(), Box<dyn Error>> {
        let text = "ATA";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 3, 1, &DNA)?;
        ans.sort();
        let mut output = vec![
            "AAA", "AAT", "ACA", "AGA", "ATA", "ATC", "ATG", "ATT", "CAT", "CTA", "GAT", "GTA",
            "TAA", "TAC", "TAG", "TAT", "TCT", "TGT", "TTA", "TTT",
        ];
        output.sort();
        assert_eq!(ans, output);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement6() -> Result<(), Box<dyn Error>> {
        let text = "AAT";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 3, 0, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AAT", "ATT"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement7() -> Result<(), Box<dyn Error>> {
        let text = "TAGCG";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 2, 1, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["CA", "CC", "GG", "TG"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches_reverse_complement8() -> Result<(), Box<dyn Error>> {
        let text = "CTTGCCGGCGCCGATTATACGATCGCGGCCGCTTGCCTTCTTTATAATGCATCGGCGCCGCGATCTTGCTATATACGTACGCTTCGCTTGCATCTTGCGCGCATTACGTACTTATCGATTACTTATCTTCGATGCCGGCCGGCATATGCCGCTTTAGCATCGATCGATCGTACTTTACGCGTATAGCCGCTTCGCTTGCCGTACGCGATGCTAGCATATGCTAGCGCTAATTACTTAT";
        let mut ans = frequent_words_with_mismatches_reverse_complement(text, 9, 3, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AGCGCCGCT", "AGCGGCGCT"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches1() -> Result<(), Box<dyn Error>> {
        let text = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
        let mut ans = frequent_words_with_mismatches(text, 4, 1, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["ATGC", "ATGT", "GATG"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches2() -> Result<(), Box<dyn Error>> {
        let text = "AGGT";
        let mut ans = frequent_words_with_mismatches(text, 2, 1, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["GG"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches3() -> Result<(), Box<dyn Error>> {
        let text = "AGGGT";
        let mut ans = frequent_words_with_mismatches(text, 2, 0, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["GG"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches4() -> Result<(), Box<dyn Error>> {
        let text = "AGGCGG";
        let mut ans = frequent_words_with_mismatches(text, 3, 0, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["AGG", "CGG", "GCG", "GGC"]);
        Ok(())
    }

    #[test]
    fn test_frequent_words_with_mismatches5() -> Result<(), Box<dyn Error>> {
        let text = "CACAGTAGGCGCCGGCACACACAGCCCCGGGCCCCGGGCCGCCCCGGGCCGGCGGCCGCCGGCGCCGGCACACCGGCACAGCCGTACCGGCACAGTAGTACCGGCCGGCCGGCACACCGGCACACCGGGTACACACCGGGGCGCACACACAGGCGGGCGCCGGGCCCCGGGCCGTACCGGGCCGCCGGCGGCCCACAGGCGCCGGCACAGTACCGGCACACACAGTAGCCCACACACAGGCGGGCGGTAGCCGGCGCACACACACACAGTAGGCGCACAGCCGCCCACACACACCGGCCGGCCGGCACAGGCGGGCGGGCGCACACACACCGGCACAGTAGTAGGCGGCCGGCGCACAGCC";
        let mut ans = frequent_words_with_mismatches(text, 10, 2, &DNA)?;
        ans.sort();
        assert_eq!(ans, vec!["GCACACAGAC", "GCGCACACAC"]);
        Ok(())
    }
}

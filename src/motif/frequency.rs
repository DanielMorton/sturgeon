use std::collections::{HashMap, HashSet};

pub fn freq_map(text: &str, k: usize) -> HashMap<String, usize> {
    // Early return for invalid cases
    if k == 0 || k > text.len() {
        return HashMap::new();
    }

    let mut word_freq = HashMap::new();

    // Use windows iterator for more idiomatic and potentially faster iteration
    text.as_bytes()
        .windows(k)
        .map(|window| String::from_utf8_lossy(window).into_owned())
        .for_each(|substr| {
            *word_freq.entry(substr).or_insert(0) += 1;
        });

    word_freq
}

pub fn frequent_words(
    text: &str,
    frequencies: &HashMap<String, usize>,
    k: usize,
    min_freq: usize,
) -> Vec<String> {
    if k == 0 || k > text.len() {
        return Vec::new();
    }

    frequencies
        .into_iter()
        .filter(|(_, &count)| count >= min_freq)
        .map(|(pattern, _)| pattern.clone())
        .collect()
}

pub fn frequent_word_set(
    text: &str,
    frequencies: &HashMap<String, usize>,
    k: usize,
    min_freq: usize,
) -> HashSet<String> {
    HashSet::from_iter(frequent_words(text, frequencies, k, min_freq))
}

pub fn most_frequent_words(text: &str, k: usize) -> Vec<String> {
    if k == 0 || k > text.len() {
        return Vec::new();
    }

    let frequencies = freq_map(text, k);

    let max_frequency = frequencies.values().max().copied().unwrap_or(0);
    frequent_words(text, &frequencies, k, max_frequency)
}

mod tests {
    use crate::motif::frequency::most_frequent_words;

    #[test]
    fn test_frequent_words1() {
        let text = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
        let mut patterns = most_frequent_words(text, 4);
        patterns.sort();
        assert_eq!(patterns, vec!["CATG", "GCAT"])
    }

    #[test]
    fn test_frequent_words2() {
        let text = "TGGTAGCGACGTTGGTCCCGCCGCTTGAGAATCTGGATGAACATAAGCTCCCACTTGGCTTATTCAGAGAACTGGTCAACACTTGTCTCTCCCAGCCAGGTCTGACCACCGGGCAACTTTTAGAGCACTATCGTGGTACAAATAATGCTGCCAC";
        let mut patterns = most_frequent_words(text, 3);
        patterns.sort();
        assert_eq!(patterns, vec!["TGG"])
    }

    #[test]
    fn test_frequent_words3() {
        let text = "AAAAA";
        let mut patterns = most_frequent_words(text, 1);
        patterns.sort();
        assert_eq!(patterns, vec!["A"])
    }

    #[test]
    fn test_frequent_words4() {
        let text = "ACACA";
        let mut patterns = most_frequent_words(text, 3);
        patterns.sort();
        assert_eq!(patterns, vec!["ACA"])
    }

    #[test]
    fn test_frequent_words5() {
        let text = "ACAC";
        let mut patterns = most_frequent_words(text, 5);
        patterns.sort();
        let ans: Vec<&str> = Vec::new();
        assert_eq!(patterns, ans)
    }

    #[test]
    fn test_frequent_words6() {
        let text = "ACACA";
        let mut patterns = most_frequent_words(text, 2);
        patterns.sort();
        assert_eq!(patterns, vec!["AC", "CA"])
    }

    #[test]
    fn test_frequent_words7() {
        let text = "CAGTGGCAGATGACATTTTGCTGGTCGACTGGTTACAACAACGCCTGGGGCTTTTGAGCAACGAGACTTTTCAATGTTGCACCGTTTGCTGCATGATATTGAAAACAATATCACCAAATAAATAACGCCTTAGTAAGTAGCTTTT";
        let mut patterns = most_frequent_words(text, 4);
        patterns.sort();
        assert_eq!(patterns, vec!["TTTT"])
    }

    #[test]
    fn test_frequent_words8() {
        let text = "CGGAAGCGAGATTCGCGTGGCGTGATTCCGGCGGGCGTGGAGAAGCGAGATTCATTCAAGCCGGGAGGCGTGGCGTGGCGTGGCGTGCGGATTCAAGCCGGCGGGCGTGATTCGAGCGGCGGATTCGAGATTCCGGGCGTGCGGGCGTGAAGCGCGTGGAGGAGGCGTGGCGTGCGGGAGGAGAAGCGAGAAGCCGGATTCAAGCAAGCATTCCGGCGGGAGATTCGCGTGGAGGCGTGGAGGCGTGGAGGCGTGCGGCGGGAGATTCAAGCCGGATTCGCGTGGAGAAGCGAGAAGCGCGTGCGGAAGCGAGGAGGAGAAGCATTCGCGTGATTCCGGGAGATTCAAGCATTCGCGTGCGGCGGGAGATTCAAGCGAGGAGGCGTGAAGCAAGCAAGCAAGCGCGTGGCGTGCGGCGGGAGAAGCAAGCGCGTGATTCGAGCGGGCGTGCGGAAGCGAGCGG";
        let mut patterns = most_frequent_words(text, 12);
        patterns.sort();
        assert_eq!(
            patterns,
            vec![
                "CGGCGGGAGATT",
                "CGGGAGATTCAA",
                "CGTGCGGCGGGA",
                "CGTGGAGGCGTG",
                "CGTGGCGTGCGG",
                "GCGTGCGGCGGG",
                "GCGTGGAGGCGT",
                "GCGTGGCGTGCG",
                "GGAGAAGCGAGA",
                "GGAGATTCAAGC",
                "GGCGGGAGATTC",
                "GGGAGATTCAAG",
                "GTGCGGCGGGAG",
                "TGCGGCGGGAGA"
            ]
        )
    }
}

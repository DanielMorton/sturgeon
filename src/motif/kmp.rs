fn build_lps_bytes<N: PartialEq + Copy>(pattern: &[N]) -> Vec<usize> {
    if pattern.is_empty() {
        return vec![];
    }

    let mut lps = vec![0; pattern.len()];
    let mut pos = 0;
    pattern[1..].iter().enumerate().for_each(|(i, &p)| {
        while pos > 0 && pattern[pos] != p {
            pos = lps[pos - 1];
        }

        if pattern[pos] == p {
            pos += 1;
            lps[i + 1] = pos;
        }
    });
    lps
}

fn build_lps(pattern: &str) -> Vec<usize> {
    build_lps_bytes(pattern.as_bytes())
}

fn kmp_bytes<N: PartialEq + Copy>(text: &[N], pattern: &[N]) -> Vec<usize> {
    if pattern.is_empty() || pattern.len() > text.len() {
        return vec![];
    }
    let lps = build_lps_bytes(pattern);
    let mut pos = 0;
    let mut matches = Vec::new();

    text.iter().enumerate().for_each(|(i, &n)| {
        while pos > 0 && pattern[pos] != n {
            pos = lps[pos - 1];
        }

        if pattern[pos] == n {
            if pos == pattern.len() - 1 {
                matches.push(i - pos);
                pos = lps[pos];
            } else {
                pos += 1
            }
        }
    });
    matches
}

fn kmp(text: &str, pattern: &str) -> Vec<usize> {
    kmp_bytes(text.as_bytes(), pattern.as_bytes())
}

fn pattern_count_bytes<N: PartialEq + Copy>(text: &[N], pattern: &[N]) -> usize {
    kmp_bytes(text, pattern).len()
}

fn pattern_count(text: &str, pattern: &str) -> usize {
    kmp(text, pattern).len()
}

mod tests {
    use crate::motif::kmp::{build_lps, kmp, pattern_count};

    #[test]
    fn test_lps1() {
        let pattern = "ABCDABD";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 0, 0, 1, 2, 0]);
    }

    #[test]
    fn test_lps2() {
        let pattern = "ABACABABC";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 1, 0, 1, 2, 3, 2, 0]);
    }

    #[test]
    fn test_lps3() {
        let pattern = "ABACABABA";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 1, 0, 1, 2, 3, 2, 3]);
    }

    #[test]
    fn test_lps4() {
        let pattern = "GCG";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 1]);
    }

    #[test]
    fn test_lps5() {
        let pattern = "CG";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0]);
    }

    #[test]
    fn test_lps6() {
        let pattern = "AAA";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 1, 2]);
    }

    #[test]
    fn test_lps7() {
        let pattern = "ACT";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 0]);
    }

    #[test]
    fn test_lps8() {
        let pattern = "AGCCTTTAG";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![0, 0, 0, 0, 0, 0, 0, 1, 2]);
    }

    #[test]
    fn test_lps_empty() {
        let pattern = "";
        let lps = build_lps(pattern);
        assert_eq!(lps, vec![]);
    }

    #[test]
    fn test_kmp1() {
        let text = "GCGCG";
        let pattern = "GCG";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![0, 2]);
    }

    #[test]
    fn test_kmp2() {
        let text = "ACGTACGTACGT";
        let pattern = "CG";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![1, 5, 9]);
    }

    #[test]
    fn test_kmp3() {
        let text = "AAAGAGTGTCTGATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAATTTTATTGACTTAGGTCACTAAATACTTTAACCAATATAGGCATAGCGCACAGACAGATAATAATTACAGAGTACACAACATCCAT";
        let pattern = "AAA";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![0, 46, 51, 74]);
    }

    #[test]
    fn test_kmp4() {
        let text = "AGCGTGCCGAAATATGCCGCCAGACCTGCTGCGGTGGCCTCGCCGACTTCACGGATGCCAAGTGCATAGAGGAAGCGAGCAAAGGTGGTTTCTTTCGCTTTATCCAGCGCGTTAACCACGTTCTGTGCCGACTTT";
        let pattern = "TTT";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![88, 92, 98, 132]);
    }

    #[test]
    fn test_kmp5() {
        let text = "GGACTTACTGACGTACG";
        let pattern = "ACT";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![2, 6]);
    }

    #[test]
    fn test_kmp6() {
        let text = "GGACTTACTGACGTACG";
        let pattern = "act";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![]);
    }

    #[test]
    fn test_kmp7() {
        let text = "GGACTTACTGACGTACG";
        let pattern = "ACTA";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![]);
    }

    #[test]
    fn test_kmp8() {
        let text = "";
        let pattern = "A";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![]);
    }

    #[test]
    fn test_kmp9() {
        let text = "";
        let pattern = "";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![]);
    }

    #[test]
    fn test_kmp10() {
        let text = "ACTG";
        let pattern = "";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(kmp_vec, vec![]);
    }

    #[test]
    fn test_kmp11() {
        let text = "CCTTTAGCCTTTTAAGCCTTTTGCAGCCTTTCAAAGAGCCTTTAGCCTTTACGGAGCCTTTAGCCTTTAAGCCTTTCTCACTAGCCTTTTTAGCCTTTGAGCCTTTATGACGAAGCCTTTAGCCTTTTGTCGTGACCTGAGCCTTTAGCCTTTACAGCCTTTCAGCCTTTAGCCTTTCTTAAAAGCCTTTTAGCCTTTTTGAGCCTTTACAGCCTTTCGAGCCTTTGAGCCTTTCCCAGCCTTTGAAGCCTTTTGGACAGAGCCTTTGCTAGCCTTTAGCCTTTTAGCCTTTAGCCTTTAGCCTTTACTTAGCCTTTTAGCCTTTATGGATAGCCTTTAGCCTTTGAGAGCCTTTGCCTAGCCTTTGAAGCCTTTTTAGCCTTTAACGAGCCTTTAGCCTTTAGCCTTTAGCCTTTAAGCCTTTAGCCTTTCGAGCCTTTCTCAGCCTTTGTAGCCTTTAGCCTTTAGAGCAGCCTTTAGCCTTTCCAGCCTTTAGCCTTTTCAGCCTTTAGCCTTTCAGCCTTTGCCCCGAGCACGTAGCCTTTACAGCCTTTAGCCTTTAGCCTTTTAGCCTTTACAGCCTTTTGAGCCTTTAGCCTTTGAAAGCCTTTTGAAGAGCCTTTCAGCCTTTCTTACTAGCCTTTGCAGCCTTTTAGCCTTTCCGAGCCTTTGATAGCCTTTGTCGGTAAGCCTTTGTAGAGCCTTTAGCCTTTAAGCCTTTGGTAAAGAGCCTTTTCAACAGCCTTTCGGAGCCTTTCGCTACAAGCCTTTTGGCCTAGCCTTTAGCCTTTCAGCCTTTCAAGAGCCTTTAGCCTTTCGCAGCCTTTATAGCCTTTCAGCCTTTCAGCCTTTAGCCTTTAGAGCCTTTGAGCCTTTCGTTATCTAAGCCTTTACTCCATAGCCTTTGAGCCTTTAGCCTTTGTCAGTCGAGCCTTTGTTCTTGAGCCTTTAGCCTTTGCAGCCTTTAGCCTTTTGTTTGTGGAGCCTTTAGCCTTTGAATACAGCCTTTAGCCTTTAGCCTTTAGCCTTTCTAGCCTTTCAGCAGCCTTTGTAGCCTTTGAACCAGCCTTTAGCCTTTTAGCCTTTTCCTTAGCCTTTCCAGCCTTTTAGTGAGCCTTTAGCCTTTGCACCAGCCTTTAGCCTTTAGCCTTTCAGCCTTTAGCCTTTCGAGCCTTTTAGCCTTTGAACAGCCTTTTGAGCCTTTGACGATATGAGCCTTTAGCCTTTTGTAGCCTTTTTTAGCCTTTGAACAGCCTTTGGAGTCAAGCCTTTACGCAGCCTTTCCAGCCTTTCAGCCTTTAGCCTTTGGTCAGCCTTTTCAGAGCCTTTGCGGTTAGCCTTTGAATAGCCTTTAAAGCCTTTCTCAGCCTTTGTAAGCCTTTAGCCTTTTAGCCTTTGTGAGCCTTTCAGCCTTTCCGAGCCTTTAGCCTTTGCCTACGGAAGCCTTTAGCCTTTGCTATCAGCTTGAGCCTTTTAGCCTTTAGTAGCAGCCTTTTAGCCTTTTAGCCTTTCAGCCTTTCTCTAGCCTTTAGCCTTTATCCGAGCCTTTACCAGCCTTTGAGCCTTTAGCCTTTATAGCCTTTATACGTAGCTAGCCTTTAGCCTTTAGAGCCTTTACCCTGTACCAGCCTTTAAGCCTTTCTCGTGAAGCCTTTAGCCTTTGAGCCTTTCGAGCCTTTAGCCTTTAGCCTTTAAGCCTTTTTGTGTGAGCCTTTAGCCTTTGGGGAGCCTTTAGCCTTTCAGCCTTTTAGCCTTTTCAAGCCTTTAGCCTTTAGCCTTTTGAGCCTTTAAAGCCTTTAGCCTTTAGGTAGCAAGCCTTTCGTTATAGCCTTTTATAAGCCTTTTTTAATGAGCCTTTAGCCTTTAGCCTTTGAGCAGCCTTTAGCCTTTAGTAGCCTTTTGATATTAGCCTTTCAGCCTTTAGCCTTTCCCCGAGCCTTTGTTAGAGCCTTTGCAGCCTTTGGAGCCTTTAGCCTTTCGGAGCCTTTAGCCTTTGGGACAGCCTTTAGCCTTTAGCCTTTGAAGCCTTTTGCAGCCTTTAAGATAGCCTTTGAGCCTTTTCAGCCTTTACAGCCTTTAAGCCTTTAGCCTTTGAGCCTTTGAGCCTTTTGAGCCTTTTAGCCTTTGTTGCAGCCTTTAGCCTTTAGCCTTTTAGCCTTTAGCCTTTAGCCTTTGAGCCTTTGAGCCTTTTAGCCTTTAGCCTTTGAGCCTTTTGGACAGCCTTTCTGAGCCTTTCGTAGCCTTTACCGCAAGCCTTTATAGCCTTTGAAGAGGAGCCTTTATAGCCTTTCAGAAGCCTTTTAAGCCTTTTCGCAGCCTTTTATCAGCCTTTAGCCTTTAGCCTTTTAGCCTTTCAGCCTTTAGCCTTTACAAGCCTTTAGCCTTTAGCCTTTATCAAGCCTTTCTAGCCTTTGAGCCTTTGTGAGCCTTTGTGTCAGCCTTTCAAGCCTTTTTAAGTACAGCCTTTACTCAGCCTTTATAGCCTTTGTCGTAAGCCTTTAGCCTTTAGCCTTTGAAAAGCCTTTACGCACAGACAAGTAGCCTTTCAGCCTTTAAGCCTTTGAGTATGTCCTTGAGCCTTTAAAAGAGCCTTTGGTAGCCTTTAGCCTTTAGCCTTTTATAGCCTTTAAGCCTTTAAGCCTTT";
        let pattern = "AGCCTTTAG";
        let kmp_vec = kmp(text, pattern);
        assert_eq!(
            kmp_vec,
            vec![
                36, 54, 113, 139, 163, 270, 285, 292, 331, 388, 395, 402, 417, 452, 459, 471, 487,
                503, 547, 554, 587, 699, 777, 803, 845, 852, 907, 943, 959, 982, 1002, 1009, 1016,
                1064, 1112, 1131, 1138, 1153, 1213, 1293, 1375, 1418, 1441, 1475, 1523, 1560, 1593,
                1600, 1648, 1672, 1679, 1708, 1726, 1759, 1766, 1791, 1798, 1851, 1858, 1876, 1883,
                1915, 1964, 1981, 2000, 2007, 2079, 2131, 2138, 2153, 2160, 2191, 2325, 2332, 2355,
                2372, 2379, 2493, 2500, 2597, 2604
            ]
        );
    }

    fn test_pattern_count1() {
        let text = "ACGTACGTACGT";
        let pattern = "CG";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 3);
    }

    fn test_pattern_count2() {
        let text = "ATGCGCGTA";
        let pattern = "GCG";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 2);
    }

    fn test_pattern_count3() {
        let text = "AAAGAGTGTCTGA";
        let pattern = "AAA";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 1);
    }

    fn test_pattern_count4() {
        let text = "AGCGTGCCGAAATTT";
        let pattern = "TTT";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 1);
    }

    fn test_pattern_count5() {
        let text = "GGACTTACTGACGTACG";
        let pattern = "ACT";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 2);
    }

    fn test_pattern_count6() {
        let text = "ATCCGATCCCATGCCCATG";
        let pattern = "CC";
        let count = pattern_count(text, pattern);
        assert_eq!(count, 5);
    }
}

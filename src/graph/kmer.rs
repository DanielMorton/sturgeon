fn make_kmer(text: &[u8], k: usize) -> Vec<Vec<u8>> {
    text.windows(k).map(|t| t.to_owned()).collect::<Vec<_>>()
}

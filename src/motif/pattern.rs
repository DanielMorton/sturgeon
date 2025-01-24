fn pattern_count(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() || pattern.len() > text.len() {
        return 0;
    }

    let pattern_bytes = pattern.as_bytes();

    // Use windows() to create an iterator of overlapping slices
    // and count the ones that match the pattern
    text.as_bytes()
        .windows(pattern.len())
        .filter(|&window| window == pattern_bytes)
        .count()
}

use std::error::Error;

fn count_breakpoints(permutation: &[i32]) -> Result<usize, Box<dyn Error>> {
    // Count the number of breakpoints
    Ok(if permutation.is_empty() {
        0
    } else {
        permutation
            .windows(2)
            .filter(|&window|
                // A breakpoint occurs when consecutive elements are not in ascending order by 1
                window[1] != window[0] + 1
            )
            .count() + if permutation[0] != 1 {1} else {0}
    })
}

mod tests {
    use std::error::Error;
    use crate::genome::breakpoints::count_breakpoints;

    #[test]
    fn test_count_breakpoints1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![3, 4, 5, -12, -8, -7, -6, 1, 2, 10, 9, -11, 13, 14];
        assert_eq!(count_breakpoints(&spectrum)?, 8);
        Ok(())
    }
}
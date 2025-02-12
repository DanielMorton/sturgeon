use std::error::Error;

fn count_breakpoints(permutation: &[i32]) -> Result<usize, Box<dyn Error>> {
    if permutation.is_empty() {
        return Ok(0);
    }

    let mut breakpoints = 0;

    // Check if the first element is not 1
    if permutation[0] != 1 {
        breakpoints += 1;
    }

    if permutation[permutation.len() - 1] != permutation.len() as i32 {
        breakpoints += 1;
    }

    // Count breakpoints in the rest of the permutation
    breakpoints += permutation
        .windows(2)
        .filter(|window| window[1] != window[0] + 1)
        .count();

    Ok(breakpoints)
}

pub(crate) fn make_breakpoints_graph<T: Clone>(
    red_edges: &[(T, T)],
    blue_edges: &[(T, T)],
) -> Result<Vec<(T, T)>, Box<dyn Error>> {
    Ok(red_edges
        .iter()
        .cloned()
        .chain(blue_edges.iter().cloned())
        .collect::<Vec<_>>())
}
#[cfg(test)]
mod tests {
    use crate::genome::breakpoints::count_breakpoints;
    use std::error::Error;

    #[test]
    fn test_count_breakpoints1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![3, 4, 5, -12, -8, -7, -6, 1, 2, 10, 9, -11, 13, 14];
        assert_eq!(count_breakpoints(&spectrum)?, 8);
        Ok(())
    }

    #[test]
    fn test_count_breakpoints2() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![1, 2, 3, 4];
        assert_eq!(count_breakpoints(&spectrum)?, 0);
        Ok(())
    }

    #[test]
    fn test_count_breakpoints3() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![-1, -2, -3, -4, -5, -6];
        assert_eq!(count_breakpoints(&spectrum)?, 7);
        Ok(())
    }

    #[test]
    fn test_count_breakpoints4() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![-1, 2, 3, 4, -5];
        assert_eq!(count_breakpoints(&spectrum)?, 4);
        Ok(())
    }

    #[test]
    fn test_count_breakpoints5() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![-5, -4, -3, -2, -1];
        assert_eq!(count_breakpoints(&spectrum)?, 2);
        Ok(())
    }
}

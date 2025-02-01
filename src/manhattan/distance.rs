use std::cmp::min;
use std::error::Error;
use std::mem::swap;

fn edit_distance(s: &str, t: &str) -> Result<usize, Box<dyn Error>> {
    // Ensure s is the shorter string to minimize memory usage
    if s.len() > t.len() {
        return edit_distance(t, s);
    }

    // We only need to store two rows - the current row and the previous row
    let mut prev_row = (0..=s.len()).collect::<Vec<_>>();
    let mut curr_row = vec![0; s.len() + 1];

    // Convert strings to bytes once
    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();

    // Process row by row
    for j in 0..t_chars.len() {
        curr_row[0] = j + 1;

        for i in 0..s_chars.len() {
            let substitution_cost = if s_chars[i] == t_chars[j] { 0 } else { 1 };
            curr_row[i + 1] = min(
                min(
                    curr_row[i] + 1,     // insertion
                    prev_row[i + 1] + 1, // deletion
                ),
                prev_row[i] + substitution_cost, // substitution
            );
        }

        // Swap rows
        swap(&mut curr_row, &mut prev_row);
    }

    Ok(prev_row[s.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() -> Result<(), Box<dyn Error>> {
        assert_eq!(edit_distance("GAGA", "GAT")?, 2);
        assert_eq!(edit_distance("AC", "AC")?, 0);
        assert_eq!(edit_distance("AT", "G")?, 2);
        assert_eq!(edit_distance("CAGACCGAGTTAG", "CGG")?, 10);
        assert_eq!(edit_distance("CGT", "CAGACGGTGACG")?, 9);
        Ok(())
    }
}

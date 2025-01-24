use crate::motif::hamming::hamming_distance;
use std::collections::HashSet;
use std::error::Error;

pub(crate) fn neighbors(
    pattern: &str,
    max_diff: usize,
    char_set: &[char],
) -> Result<HashSet<String>, Box<dyn Error>> {
    // Base case: if no differences are allowed, return the pattern itself.
    if max_diff == 0 {
        return Ok(HashSet::from([pattern.to_string()]));
    }

    // Base case: if the pattern has length 1, return all possible single characters.
    if pattern.len() == 1 {
        return Ok(char_set.iter().map(|&c| c.to_string()).collect());
    }

    // Recursive case: Generate neighbors for the substring and expand with character set.
    let suffix_neighbors = neighbors(&pattern[1..], max_diff, char_set)?;

    let mut result = HashSet::new();
    for neighbor in &suffix_neighbors {
        // Calculate Hamming distance once for reuse.
        let hd = hamming_distance(neighbor, &pattern[1..])?;
        for &c in char_set {
            let new_pattern = format!("{}{}", c, neighbor);
            if hd < max_diff {
                result.insert(new_pattern);
            }
        }
        // Include the original character if max_diff is not exceeded.
        if hd <= max_diff {
            result.insert(format!("{}{}", &pattern[0..1], neighbor));
        }
    }

    Ok(result)
}

mod test {
    use crate::motif::neighbors::neighbors;
    use crate::utils::DNA;
    use std::collections::HashSet;
    use std::error::Error;
    use std::fs;

    #[test]
    fn test_neighbors1() -> Result<(), Box<dyn Error>> {
        let pattern = "ACG";

        let ans = neighbors(pattern, 1, &DNA)?;
        assert_eq!(
            ans,
            HashSet::from(
                ["ACG", "ACC", "ACT", "ACA", "ATG", "AAG", "AGG", "CCG", "TCG", "GCG"]
                    .map(String::from)
                    .to_owned()
            )
        );
        Ok(())
    }

    #[test]
    fn test_neighbors2() -> Result<(), Box<dyn Error>> {
        let pattern = "AGA";
        let ans = neighbors(pattern, 0, &DNA)?;
        assert_eq!(ans, HashSet::from(["AGA"].map(String::from).to_owned()));
        Ok(())
    }

    #[test]
    fn test_neighbors3() -> Result<(), Box<dyn Error>> {
        let pattern = "AAA";
        let ans = neighbors(pattern, 1, &DNA)?;
        assert_eq!(
            ans,
            HashSet::from(
                ["AAA", "AAC", "AAG", "AAT", "ACA", "AGA", "ATA", "CAA", "GAA", "TAA"]
                    .map(String::from)
                    .to_owned()
            )
        );
        Ok(())
    }

    #[test]
    fn test_neighbors4() -> Result<(), Box<dyn Error>> {
        let pattern = "A";
        let ans = neighbors(pattern, 1, &DNA)?;
        assert_eq!(
            ans,
            HashSet::from(["A", "C", "G", "T"].map(String::from).to_owned())
        );
        Ok(())
    }

    #[test]
    fn test_neighbors7() -> Result<(), Box<dyn Error>> {
        let pattern = "GGCCCAGAG";
        let ans = neighbors(pattern, 3, &DNA)?;
        let output = match fs::read_to_string("output_neighbors.txt") {
            Ok(g) => g.split("\n").map(String::from).collect::<HashSet<_>>(),
            Err(e) => panic!("{}", e),
        };
        assert_eq!(ans, output);
        Ok(())
    }
}

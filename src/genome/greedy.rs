use std::error::Error;

fn sort_reverse(p: &mut [i32]) -> Result<(), Box<dyn Error>> {
    let (mut left, mut right) = (0, p.len() - 1);
    while left < right {
        p.swap(left, right);
        p[left] = -p[left];
        p[right] = -p[right];
        left += 1;
        right -= 1;
    }

    if left == right {
        p[left] = -p[left];
    }
    Ok(())
}

fn greedy_sorting(p: &[i32]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut sorting = Vec::new();
    let mut p = p.to_vec(); // Create a mutable copy

    for i in 1..=p.len() {
        let mut right = i - 1;

        // Find the correct position or its negation
        while right < p.len() && p[right] != i as i32 && p[right] != -(i as i32) {
            right += 1;
        }

        if right >= i || p[right] == -(i as i32) {
            // Reverse and sort the subslice
            let _ = sort_reverse(&mut p[i - 1..=right]);
            sorting.push(p.clone());

            // Check if we need an additional reversal
            if p[i - 1] == -(i as i32) {
                sort_reverse(&mut p[i - 1..i])?;
                sorting.push(p.clone());
            }
        }
    }

    Ok(sorting)
}
#[cfg(test)]
mod tests {
    use crate::genome::greedy::greedy_sorting;
    use std::error::Error;

    #[test]
    fn test_greedy_sorting1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![-3, 4, 1, 5, -2];
        let sorting = greedy_sorting(&spectrum)?;
        let ans = vec![
            vec![-1, -4, 3, 5, -2],
            vec![1, -4, 3, 5, -2],
            vec![1, 2, -5, -3, 4],
            vec![1, 2, 3, 5, 4],
            vec![1, 2, 3, -4, -5],
            vec![1, 2, 3, 4, -5],
            vec![1, 2, 3, 4, 5],
        ];
        assert_eq!(sorting, ans);
        Ok(())
    }

    #[test]
    fn test_greedy_sorting2() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![1, 2, -3, 4, 5];
        let sorting = greedy_sorting(&spectrum)?;
        let ans = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(sorting, ans);
        Ok(())
    }

    #[test]
    fn test_greedy_sorting3() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![-1, -2, -5, -4, -3];
        let sorting = greedy_sorting(&spectrum)?;
        let ans = vec![
            vec![1, -2, -5, -4, -3],
            vec![1, 2, -5, -4, -3],
            vec![1, 2, 3, 4, 5],
        ];
        assert_eq!(sorting, ans);
        Ok(())
    }

    #[test]
    fn test_greedy_sorting4() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![3, 1, 2, 4];
        let sorting = greedy_sorting(&spectrum)?;
        let ans = vec![
            vec![-1, -3, 2, 4],
            vec![1, -3, 2, 4],
            vec![1, -2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        assert_eq!(sorting, ans);
        Ok(())
    }

    #[test]
    fn test_greedy_sorting5() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![3, 2, 1];
        let sorting = greedy_sorting(&spectrum)?;
        let ans = vec![
            vec![-1, -2, -3],
            vec![1, -2, -3],
            vec![1, 2, -3],
            vec![1, 2, 3],
        ];
        assert_eq!(sorting, ans);
        Ok(())
    }
}

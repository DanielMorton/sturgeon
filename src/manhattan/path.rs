use std::error::Error;
use std::fmt::Debug;
use num::Num;


fn make_grid<T:Num + Copy>(rows: usize, cols: usize) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    // Preallocate with capacity for better performance
    Ok(vec![vec![T::zero(); cols]; rows])
}

pub fn longest_path_length<T:Num + Debug + Copy + Ord>(m: usize, n: usize, down: &[Vec<T>], right: &[Vec<T>]) -> Result<T, Box<dyn Error>> {
    // Create grid with exact size needed
    let mut ans = make_grid(m + 1, n + 1)?;

    // Initialize first column using down values
    for i in 1..=m {
        ans[i][0] = ans[i-1][0] + down[i-1][0];
    }

    // Initialize first row using right values
    for j in 1..=n {
        ans[0][j] = ans[0][j-1] + right[0][j-1];
    }

    // Fill the rest of the grid
    for i in 1..=m {
        for j in 1..=n {
            // Calculate potential paths
            let down_path: T = ans[i-1][j] + down[i-1][j];
            let right_path: T = ans[i][j-1] + right[i][j-1];

            // Use max for cleaner comparison
            ans[i][j] = down_path.max(right_path);
        }
    }
    Ok(ans[m][n])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path1() -> Result<(), Box<dyn Error>>{
        let down = vec![
            vec![1, 0, 2, 4, 3],
            vec![4, 6, 5, 2, 1],
            vec![4, 4, 5, 2, 1],
            vec![5, 6, 8, 5, 3],
        ];
        let right = vec![
            vec![3, 2, 4, 0],
            vec![3, 2, 4, 2],
            vec![0, 7, 3, 3],
            vec![3, 3, 0, 2],
            vec![1, 3, 2, 2],
        ];

        let result = longest_path_length(4, 4, &down, &right)?;
        assert_eq!(result, 34); // Example test case
        Ok(())
    }

    #[test]
    fn test_longest_path2() -> Result<(), Box<dyn Error>>{
        let down = vec![
            vec![20, 0, 0],
            vec![20, 0, 0]
        ];
        let right = vec![
            vec![0, 0],
            vec![0, 0],
            vec![10, 10]
        ];

        let result = longest_path_length(2, 2, &down, &right)?;
        assert_eq!(result, 60); // Example test case
        Ok(())
    }

    #[test]
    fn test_longest_path3() -> Result<(), Box<dyn Error>>{
        let down = vec![
            vec![0, 0, 20],
            vec![0, 0, 20]
        ];
        let right = vec![
            vec![10, 10],
            vec![0, 0],
            vec![0, 0]
        ];

        let result = longest_path_length(2, 2, &down, &right)?;
        assert_eq!(result, 60); // Example test case
        Ok(())
    }

    #[test]
    fn test_longest_path4() -> Result<(), Box<dyn Error>>{
        let down = vec![
            vec![20, 0, 0],
            vec![0, 0, 0]
        ];
        let right = vec![
            vec![0, 30],
            vec![0, 0],
            vec![0, 0]
        ];

        let result = longest_path_length(2, 2, &down, &right)?;
        assert_eq!(result, 30); // Example test case
        Ok(())
    }

    #[test]
    fn test_longest_path5() -> Result<(), Box<dyn Error>>{
        let down = vec![
            vec![20,  5,  0, 10],
            vec![ 0,  5, 10,  0],
            vec![10, 10,  0, 15],
            vec![ 0, 20, 20, 25],
            vec![30, 10,  5, 30]
        ];
        let right = vec![
            vec![ 0, 30, 15],
            vec![10, 20, 10],
            vec![10, 10, 20],
            vec![20, 25, 30],
            vec![15, 35, 40],
            vec![15, 10, 25]
        ];

        let result = longest_path_length(5, 3, &down, &right)?;
        assert_eq!(result, 175); // Example test case
        Ok(())
    }
}
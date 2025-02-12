use std::error::Error;

pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    // Handle empty input case
    if matrix.is_empty() {
        return Ok(Vec::new());
    }

    let cols = matrix[0].len();

    // Create a new vector with transposed dimensions
    let mut transposed = vec![];

    // Fill the transposed vector
    for j in 0..cols {
        let mut transposed_row = vec![];
        for row in matrix.iter() {
            transposed_row.push(row[j].clone());
        }
        transposed.push(transposed_row)
    }

    Ok(transposed)
}
#[cfg(test)]
mod tests {
    use crate::utils::transpose;
    use std::error::Error;

    #[test]
    fn test_global_alignment2() -> Result<(), Box<dyn Error>> {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            transpose(&matrix)?,
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
        Ok(())
    }
}

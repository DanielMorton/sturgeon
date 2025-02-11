use crate::genome::two_break_sorting;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let p = vec![vec![1, 2, 3, 4, 5, 6]];
    let q = vec![vec![1, -3, -6, -5], vec![2, -4]];
    let ans = vec![
        vec![vec![1, 2, 3, 4, 5, 6]],
        vec![vec![1, 2, 3, 4, -6, -5]],
        vec![vec![1, 2, -4, -3, -6, -5]],
        vec![vec![1, -3, -6, -5], vec![2, -4]],
    ];

    assert_eq!(two_break_sorting(&p, &q)?, ans);
    Ok(())
}

use std::error::Error;

pub(crate) fn find_closest_clusters(d: &[Vec<f64>]) -> Result<(usize, usize, f64), Box<dyn Error>> {
    let mut min_i = 0;
    let mut min_j = 1;
    let mut min_dist = d[min_i][min_j];

    for i in 0..d.len() {
        for j in (i + 1)..d.len() {
            if d[i][j] < min_dist {
                min_i = i;
                min_j = j;
                min_dist = d[i][j];
            }
        }
    }

    Ok((min_i, min_j, min_dist))
}

use std::fmt::Debug;
#[derive(Debug, Clone, PartialEq)]
pub struct AlignmentResult<T> {
    score: T,
    alignment1: String,
    alignment2: String,
}
impl<T> AlignmentResult<T> {
    pub fn new(score: T, alignment1: &str, alignment2: &str) -> Self {
        AlignmentResult {
            score,
            alignment1: alignment1.to_owned(),
            alignment2: alignment2.to_owned(),
        }
    }
}

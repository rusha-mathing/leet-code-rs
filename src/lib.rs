pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, other| {
                acc.chars()
                    .zip(other.chars())
                    .take_while(|(i, j)| i == j)
                    .map(|(i, _)| i)
                    .collect()
            })
            .expect("strs len must be great than or equal to 1")
    }
}

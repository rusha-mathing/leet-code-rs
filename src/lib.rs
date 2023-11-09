pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_chars(s.chars(), p.chars())
    }

    pub fn is_match_chars(s: impl Iterator<Item = char>, p: impl Iterator<Item = char>) -> bool {
        todo!()
    }
}

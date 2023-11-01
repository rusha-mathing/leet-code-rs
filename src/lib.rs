pub struct Solution();

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        (0..s.len())
            .map(|i| {
                (i..s.len())
                    .map(|j| &s[i..=j])
                    .filter(|&arg| Self::is_palindrome(arg))
                    .max_by_key(|x| x.len())
                    .unwrap_or_default()
            })
            .max_by_key(|x| x.len())
            .unwrap_or_default()
            .to_owned()
    }

    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
}

use std::collections::HashMap;

pub struct Solution();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = HashMap::new();
        let mut i = 0;
        let mut ans = 0;
        for (j, ch) in s.char_indices() {
            if hash.contains_key(&ch) {
                i = i.max(hash.insert(ch, j).unwrap() + 1);
            } else {
                hash.insert(ch, j);
            }
            if j - i + 1 > ans {
                ans = j - i + 1;
            }
        }
        ans as i32
    }
}

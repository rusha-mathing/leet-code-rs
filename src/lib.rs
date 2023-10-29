use std::collections::HashMap;

pub struct Solution();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = HashMap::new();
        let mut i = 0;
        s.char_indices()
            .map(|(j, ch)| {
                let temp = hash.insert(ch, j).map_or(0, |x| x + 1);
                i = usize::max(i, temp);
                j - i + 1
            })
            .max()
            .unwrap_or(0) as i32
    }
}

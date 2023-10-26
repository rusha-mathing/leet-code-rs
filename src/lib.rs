use std::collections::HashMap;

pub struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            if let Some(&j) = map.get(&(target - x)) {
                return vec![i as i32, j as i32];
            }
            map.insert(x, i);
        }
        unreachable!("Constraints isn't met")
    }
}

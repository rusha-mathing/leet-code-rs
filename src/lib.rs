pub struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate().skip(i + 1) {
                if x + y == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
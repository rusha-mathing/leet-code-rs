use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut cash_memory: HashMap<i32, usize> = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate().skip(i + 1) {
                for z in nums.iter().skip(j + 1) {
                    let remaind = target - (x + y + z);
                    if cash_memory.contains_key(&remaind) {
                        result.insert({
                            let mut quadruplets = vec![*x, *y, *z, remaind];
                            quadruplets.sort();
                            quadruplets
                        });
                    }
                }
            }
            cash_memory.insert(*x, i);
        }
        result.into_iter().collect()
    }
}

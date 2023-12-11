use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut cash_memory: HashMap<i64, usize> = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate().skip(i + 1) {
                for z in nums.iter().skip(j + 1) {
                    let remaind = target as i64 - (*x as i64 + *y as i64 + *z as i64);
                    if cash_memory.contains_key(&remaind) {
                        result.insert({
                            let mut quadruplets = vec![*x, *y, *z, remaind as i32];
                            quadruplets.sort();
                            quadruplets
                        });
                    }
                }
            }
            cash_memory.insert(*x as i64, i);
        }
        result.into_iter().collect()
    }
}

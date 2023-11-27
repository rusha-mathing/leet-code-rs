pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();

        let mut main_pointer = nums.iter();
        let mut current = main_pointer.next();
        while let Some(val) = current {
            let mut remaind = main_pointer.clone();
            let (mut left, mut right) = (remaind.next(), remaind.next_back());
            while let (Some(i), Some(j)) = (left, right) {
                let sum = i + j + val;
                match sum.cmp(&0) {
                    Ordering::Less => left = remaind.next(),
                    Ordering::Greater => right = remaind.next_back(),
                    Ordering::Equal => {
                        result.push(vec![*val, *i, *j]);
                        while left == Some(i) {
                            left = remaind.next()
                        }
                        while right == Some(j) {
                            right = remaind.next_back();
                        }
                    }
                }
            }
            while current == Some(val) {
                current = main_pointer.next();
            }
        }
        result
    }
}

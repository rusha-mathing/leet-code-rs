pub struct Solution;

use std::cmp::Ordering;

const MAX_VALUE: i32 = 100000;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut result = MAX_VALUE;

        let mut main_pointer = nums.iter();
        let mut current = main_pointer.next();
        while let Some(val) = current {
            let mut remaind = main_pointer.clone();
            let (mut left, mut right) = (remaind.next(), remaind.next_back());
            while let Some((i, j)) = left.zip(right) {
                let sum = i + j + val;
                result = std::cmp::min_by_key(result, sum, |x| x.abs_diff(target));
                match sum.cmp(&target) {
                    Ordering::Less => left = remaind.next(),
                    Ordering::Greater => right = remaind.next_back(),
                    Ordering::Equal => {
                        while left == Some(i) {
                            left = remaind.next()
                        }
                        while right == Some(j) {
                            right = remaind.next_back();
                        }
                    }
                }
            }
            while current == Some(&val) {
                current = main_pointer.next();
            }
        }
        result
    }
}

pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let (mut size, mut iter) = (heights.len() - 1, heights.into_iter());
        let (mut left, mut right) = (iter.next(), iter.next_back());
        let mut ans = 0;
        while let (Some(a), Some(b)) = (left, right) {
            ans = ans.max(a.min(b) * size as i32);
            if a < b {
                left = iter.next();
            } else {
                right = iter.next_back();
            }
            size -= 1;
        }
        ans
    }
}

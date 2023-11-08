pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Self::reverse_result(x).unwrap_or_default()
    }

    fn reverse_result(x: i32) -> Option<i32> {
        let (mut current, mut total): (i32, i32) = (x.checked_abs()?, 0);
        while current > 0 {
            total = total.checked_mul(10)?.checked_add(current % 10)?;
            current /= 10;
        }
        if x < 0 {
            total = -total;
        }
        Some(total)
    }
}

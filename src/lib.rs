pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x <= 0 {
            return x == 0;
        }
        let (mut rev, mut temp): (i32, i32) = (0, x);
        while temp > 0 {
            rev = rev.saturating_mul(10).saturating_add(temp % 10);
            temp /= 10;
        }
        rev == x
    }
}
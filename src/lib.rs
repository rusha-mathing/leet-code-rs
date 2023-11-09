pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x == 0 {
            return true;
        } 
        let mut digits = Vec::with_capacity(x.checked_ilog10().unwrap() as usize);
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }
        digits.iter().eq(digits.iter().rev())
    }
}
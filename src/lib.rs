pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x <= 0 {
            return x == 0;
        }
        let (mut rev, mut temp): (i32, i32) = (0, x);
        while temp > 0 {
            rev = match rev.checked_mul(10).and_then(|x| x.checked_add(temp % 10)) {
                Some(x) => x,
                None => return false,
            };
            temp /= 10;
        }
        rev == x
    }
}
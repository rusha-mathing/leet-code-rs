pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut read = s
            .trim()
            .chars()
            .take_while(|&x| x == '+' || x == '-' || x.is_digit(10))
            .peekable();

        let mut sign = 1;
        if let Some(&first) = read.peek().filter(|&&x| x == '+' || x == '-') {
            read.next();
            if first == '-' {
                sign = -1
            }
        }
        read.take_while(|ch| ch.is_digit(10))
            .map(|ch| ch.to_digit(10).unwrap() as i32 * sign)
            .reduce(|total, digit| total.saturating_mul(10).saturating_add(digit))
            .unwrap_or(0)
    }
}

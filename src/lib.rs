pub struct Solution;

const LETTERS_BY_DIGIT: &[&str] = &[
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Option<Vec<String>> = None;
        for ch in digits.chars() {
            result = Some(
                result
                    .unwrap_or(vec![String::new()])
                    .into_iter()
                    .map(|s| Self::digit_and_str_product(ch, s.as_str()))
                    .flatten()
                    .collect(),
            );
        }
        result.unwrap_or_default()
    }

    fn digit_and_str_product(digit: char, preffix: &str) -> Vec<String> {
        let digit = digit.to_digit(10).expect("Digits must contain digit!") as usize;
        LETTERS_BY_DIGIT
            .get(digit)
            .unwrap()
            .chars()
            .map(|ch| {
                let mut result = preffix.to_owned();
                result.push(ch);
                result
            })
            .collect()
    }
}

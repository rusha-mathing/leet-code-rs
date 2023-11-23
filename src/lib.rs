pub struct Solution;
const UNITS: &[(&str, i32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Solution {
    pub fn roman_to_int(num: String) -> i32 {
        let mut temp = num.as_str();
        UNITS
            .iter()
            .rev()
            .map(|(unit, value)| {
                let mut result = 0;
                while let Some(new) = temp.strip_suffix(unit) {
                    temp = new;
                    result += value;
                }
                result
            })
            .fold(0, |a, b| a + b)
    }
}

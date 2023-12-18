pub struct Solution;

const OPEN_BRACKETS: &str = "([{";
const CLOSE_BRACKETS: &str = ")]}";

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut stack = Vec::new();

        for ch in s.chars() {
            if OPEN_BRACKETS.contains(ch) {
                stack.push(ch);
            } else if CLOSE_BRACKETS.contains(ch) {
                let open_bracket = match ch {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    unknown => unimplemented!("Unknown close bracket '{unknown}'"),
                };
                if stack.pop() != Some(open_bracket) {
                    return false;
                }
            } else {
                panic!("s must consists of parentheses only '()[]{{}}'");
            }
        }

        stack.is_empty()
    }
}

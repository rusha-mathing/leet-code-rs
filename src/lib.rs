pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        recursion(n, n, String::new(), &mut result);
        return result;

        fn recursion(open: i32, close: i32, mut current: String, result: &mut Vec<String>) {
            if open < 0 || close < 0 || open > close {
                return;
            }
            if (open, close) == (0, 0) {
                result.push(current);
                return;
            } 
            let mut clone = current.clone();
            clone.push(')');
            current.push('(');
            recursion(open - 1, close, current, result);
            recursion(open, close - 1, clone, result)
        }
    }
}

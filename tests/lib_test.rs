#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(n: i32, except: &mut [&str]) {
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        except.sort();
        assert_eq!(Solution::generate_parenthesis(n), except)
    }

    #[test]
    fn test_1() {
        helper(1, &mut ["()"])
    }

    #[test]
    fn test_2() {
        helper(2, &mut ["(())", "()()"])
    }

    #[test]
    fn test_3() {
        helper(3, &mut ["((()))", "(()())", "(())()", "()(())", "()()()"])
    }

    #[test]
    fn test_4() {
        helper(
            4,
            &mut [
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
            ],
        )
    }
}


#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(s: &str, val: bool) {
        assert_eq!(Solution::is_valid(s.into()), val)
    } 

    #[test]
    fn test_1() {
        helper("()", true)
    }

    #[test]
    fn test_2() {
        helper("()[]{}", true)
    }

    #[test]
    fn test_3() {
        helper("(]", false)
    }

    #[test]
    fn test_4() {
        helper("((", false)   
    }

    #[test]
    fn test_5() {
        helper("()()()", true);
    }

    #[test]
    fn test_6() {
        helper("((((((()))))))", true)
    }

    #[test]
    fn test_7() {
        helper("([{[{[((({})))]}]}])", true)
    }

    #[test]
    fn test_8() {
        helper("(()())[(){}]", true)
    }

    #[test]
    fn test_9() {
        helper("([)]", false);
    }

    #[test]
    fn test_10() {
        helper("([])", true)
    }

    #[test]
    fn test_11() {
        helper("(()", false);
    }

}
#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::is_palindrome(12321), true);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::is_palindrome(123456789), false);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::is_palindrome(111), true);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::is_palindrome(987654321), false);
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(1001), true);
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::is_palindrome(1234321), true);
    }

    #[test]
    fn test_11() {
        assert_eq!(Solution::is_palindrome(123454321), true);
    }

    #[test]
    fn test_12() {
        assert_eq!(Solution::is_palindrome(135797531), true);
    }

    #[test]
    fn test_13() {
        assert_eq!(Solution::is_palindrome(135798531), false);
    }

    #[test]
    fn test_14() {
        assert_eq!(Solution::is_palindrome(1000021), false);
    }

    #[test]
    fn test_15() {
        assert_eq!(Solution::is_palindrome(123210), false);
    }

    #[test]
    fn test_16() {
        assert_eq!(Solution::is_palindrome(987654321), false);
    }

    #[test]
    fn test_17() {
        assert_eq!(Solution::is_palindrome(121212121), true);
    }

    #[test]
    fn test_18() {
        assert_eq!(Solution::is_palindrome(9876789), true);
    }

    #[test]
    fn test_19() {
        assert_eq!(Solution::is_palindrome(1234543210), false);
    }

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_palindrome(1001001), true);
    }
}
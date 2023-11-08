#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        helper("42", 42);
    }

    #[test]
    fn test_2() {
        helper("   -42", -42);
    }

    #[test]
    fn test_3() {
        helper("4193 with words", 4193);
    }

    #[test]
    fn test_4() {
        helper("words and 987", 0);
    }

    #[test]
    fn test_5() {
        helper("-91283472332", -2147483648);
    }

    #[test]
    fn test_6() {
        helper("3.14159", 3);
    }

    #[test]
    fn test_7() {
        helper("  -0012a42", -12);
    }

    #[test]
    fn test_8() {
        helper("+1", 1);
    }

    #[test]
    fn test_9() {
        helper("21474836460", 2147483647);
    }

    #[test]
    fn test_10() {
        helper("1345 with words", 1345);
    }

    #[test]
    fn test_11() {
        helper("-9999999999999999999999999", -2147483648);
    }

    #[test]
    fn test_12() {
        helper("9999999999999999999999999", 2147483647);
    }

    #[test]
    fn test_13() {
        helper("-2147483648", -2147483648);
    }

    #[test]
    fn test_14() {
        helper("-2147483647", -2147483647);
    }

    #[test]
    fn test_15() {
        helper("2147483647", 2147483647);
    }

    #[test]
    fn test_16() {
        helper("9223372036854775808", 2147483647);
    }

    #[test]
    fn test_17() {
        helper("-9223372036854775808", -2147483648);
    }

    #[test]
    fn test_18() {
        helper("00000", 0);
    }

    #[test]
    fn test_19() {
        helper("        -42", -42);
    }

    #[test]
    fn test_20() {
        helper("42 this is a test", 42);
    }

    #[test]
    fn test_21() {
        helper("9223372036854775809", 2147483647);
    }

    #[test]
    fn test_22() {
        helper("-9223372036854775809", -2147483648);
    }

    #[test]
    fn test_23() {
        helper("92233720368547758070", 2147483647);
    }

    #[test]
    fn test_24() {
        helper("-92233720368547758070", -2147483648);
    }

    #[test]
    fn test_25() {
        helper("  words -42", 0);
    }

    fn helper(s: &str, expected: i32) {
        assert_eq!(Solution::my_atoi(s.to_string()), expected)
    }
}

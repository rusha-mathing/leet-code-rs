#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(s: &str, num_rows: i32, ans: &str) {
        assert_eq!(Solution::convert(s.to_string(), num_rows), ans);
    }

    #[test]
    fn test_1() {
        helper("PAYPALISHIRUNG", 3, "PAHNAPLSIIGYIR");
    }
    
    #[test]
    fn test_2() {
        helper("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    }
    
    #[test]
    fn test_3() {
        helper("ABCDE", 1, "ABCDE");
    }
    
    #[test]
    fn test_4() {
        helper("A", 1, "A");
    }
    
    #[test]
    fn test_5() {
        helper("AB", 1, "AB");
    }
    
    #[test]
    fn test_6() {
        helper("ABC", 2, "ACB");
    }
    
    #[test]
    fn test_7() {
        helper("ABCDEF", 3, "AEBDFC");
    }
    
    #[test]
    fn test_8() {
        helper("ABCDEF", 4, "ABFCED");
    }
    
    #[test]
    fn test_9() {
        helper("PAYPALISHIRING", 5, "PHASIYIRPLIGAN");
    }
    
    #[test]
    fn test_10() {
        helper("LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG");
    }
    
    #[test]
    fn test_11() {
        helper("SIXZIGZAGCONVERSION", 6, "SOICNXGVNZAEOIZRIGS");
    }
    
    #[test]
    fn test_12() {
        helper("RUSTLANG", 2, "RSLNUTAG");
    }
    
    #[test]
    fn test_13() {
        helper("ZIGZAG", 3, "ZZIAGG");
    }
    
    #[test]
    fn test_14() {
        helper("PROGRAMMING", 5, "PIRMNOMGGAR");
    }
    
    #[test]
    fn test_15() {
        helper("HELLO", 1, "HELLO");
    }
    
    #[test]
    fn test_16() {
        helper("HELLO", 10, "HELLO");
    }
    
    #[test]
    fn test_17() {
        helper("", 2, "");
    }
    
    #[test]
    fn test_18() {
        helper("A", 3, "A");
    }
    
    #[test]
    fn test_19() {
        helper("AB", 3, "AB");
    }
    
    #[test]
    fn test_20() {
        helper("ABC", 5, "ABC");
    }
}

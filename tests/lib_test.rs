
#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(Solution::reverse(-321), -123);
    }
    
    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse(120), 21);
    }
    
    #[test]
    fn test_4() {
        assert_eq!(Solution::reverse(0), 0);
    }
    
    #[test]
    fn test_5() {
        assert_eq!(Solution::reverse(5), 5);
    }
    
    #[test]
    fn test_6() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
    
    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(-1463847414), 0);
    }
    
    #[test]
    fn test_8() {
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
    
    #[test]
    fn test_9() {
        assert_eq!(Solution::reverse(1563847412), 0);
    }
    
    #[test]
    fn test_10() {
        assert_eq!(Solution::reverse(-1563847412), 0);
    }
    
    #[test]
    fn test_11() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }
    
    #[test]
    fn test_12() {
        assert_eq!(Solution::reverse(-1563847427), 0);
    }
    
    #[test]
    fn test_13() {
        assert_eq!(Solution::reverse(1463847412), 2147483641);
    }
    
    #[test]
    fn test_14() {
        assert_eq!(Solution::reverse(1463847413), 0);
    }
    
    #[test]
    fn test_15() {
        assert_eq!(Solution::reverse(1463847421), 1247483641);
    }
    
    #[test]
    fn test_16() {
        assert_eq!(Solution::reverse(1000000003), 0);
    }
    
    #[test]
    fn test_17() {
        assert_eq!(Solution::reverse(10), 1);
    }
    
    #[test]
    fn test_18() {
        assert_eq!(Solution::reverse(-10), -1);
    }
    
    #[test]
    fn test_19() {
        assert_eq!(Solution::reverse(1000000000), 1);
    }
    
    #[test]
    fn test_20() {
        assert_eq!(Solution::reverse(-1000000003), 0);
    }
    
    #[test]
    fn test_21() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
    
    #[test]
    fn test_22() {
        assert_eq!(Solution::reverse(-1534236469), 0);
    }
    
    #[test]
    fn test_23() {
        assert_eq!(Solution::reverse(1563847412), 0);
    }
    
    #[test]
    fn test_24() {
        assert_eq!(Solution::reverse(-1563847412), 0);
    }
    
    #[test]
    fn test_25() {
        assert_eq!(Solution::reverse(2147483646), 0);
    }
}
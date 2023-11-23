
#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
    
    #[test]
    fn test_3() {
        assert_eq!(Solution::max_area(vec![4,3,2,1,4]), 16);
    }
    
    #[test]
    fn test_4() {
        assert_eq!(Solution::max_area(vec![1,2,1]), 2);
    }
    
    #[test]
    fn test_5() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,25,5,7,6,10,23,12,2,0,32,13,7]), 225);
    }
    
    #[test]
    fn test_6() {
        assert_eq!(Solution::max_area(vec![2,3,4,5,18,17,6]), 17);
    }
    
    #[test]
    fn test_7() {
        assert_eq!(Solution::max_area(vec![10,9,8,7,6,5,4,3,2,1]), 25);
    }
    
    #[test]
    fn test_8() {
        assert_eq!(Solution::max_area(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]), 14);
    }
    
    #[test]
    fn test_9() {
        assert_eq!(Solution::max_area(vec![3,1,2,4,5]), 12);
    }
    
    #[test]
    fn test_10() {
        assert_eq!(Solution::max_area(vec![3,2,1,4,5]), 12);
    }
    
    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1,1,1,1,2,2,2,2,1,1,1,1]), 11);
    }
    
    #[test]
    fn test_12() {
        assert_eq!(Solution::max_area(vec![10,10,10,10,10,10,10,10,10,1,1,1,1,1,1]), 80);
    }
    
    #[test]
    fn test_13() {
        assert_eq!(Solution::max_area(vec![10,10,10,10,10,10,10,10,1,1,1,1,1,1,1]), 70);
    }
    
    #[test]
    fn test_14() {
        assert_eq!(Solution::max_area(vec![1,2,6,12,15,8,6]), 24);
    }
    
    #[test]
    fn test_15() {
        assert_eq!(Solution::max_area(vec![0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0]), 0);
    }
    
    #[test]
    fn test_16() {
        assert_eq!(Solution::max_area(vec![0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1]), 5);
    }
    
    #[test]
    fn test_17() {
        assert_eq!(Solution::max_area(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,100,1,1,1,1,1,1,1]), 31);
    }
    
    #[test]
    fn test_18() {
        assert_eq!(Solution::max_area(vec![3,5,4,2]), 6);
    }
    
    #[test]
    fn test_19() {
        assert_eq!(Solution::max_area(vec![3,4,5,6,7,8]), 16);
    }
    
    #[test]
    fn test_20() {
        assert_eq!(Solution::max_area(vec![3,1,3,3,4]), 12);
    }
    
    #[test]
    fn test_21() {
        assert_eq!(Solution::max_area(vec![2,2,3,1,1,1,2,4]), 15);
    }
    
    #[test]
    fn test_22() {
        assert_eq!(Solution::max_area(vec![3,2,1,3]), 9);
    }
    
    #[test]
    fn test_23() {
        assert_eq!(Solution::max_area(vec![2,3,4,5,18,17,6]), 17);
    }
    
    #[test]
    fn test_24() {
        assert_eq!(Solution::max_area(vec![1,1,5,4,6,2,1,3]), 15);
    }
    
    #[test]
    fn test_25() {
        assert_eq!(Solution::max_area(vec![3,6,8,7,7]), 18);
    }
    }
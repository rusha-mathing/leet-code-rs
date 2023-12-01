#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 3, 4, 5, 6], 10), 10);
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::three_sum_closest(vec![-3, -2, -1, 0, 1, 2, 3], -4),
            -4
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::three_sum_closest(vec![4, 0, 1, 2, 8, 7, 9], 8), 8);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 0, 1, 2, -1, -4], 1), 1);
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Solution::three_sum_closest(std::iter::repeat(0).take(500).collect(), 1),
            0
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 2, 0], -100), 2);
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::three_sum_closest(vec![-10, 2, 3, 4, 5, 7], 10), 10);
    }

    #[test]
    fn test_11() {
        assert_eq!(
            Solution::three_sum_closest(vec![-3, -2, -1, 0, 1, 2, 3], -4),
            -4
        );
    }

    #[test]
    fn test_12() {
        assert_eq!(Solution::three_sum_closest(vec![4, 0, 1, 2, 8, 7, 9], 8), 8);
    }

    #[test]
    fn test_13() {
        assert_eq!(Solution::three_sum_closest(vec![4, 5, 6, 7, 8, 9], 10), 15);
    }

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::three_sum_closest(vec![10, 11, 12, 13, 14, 15], 30),
            33
        );
    }

    #[test]
    fn test_15() {
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3, -4, -5, -6], 1), 1);
    }

    #[test]
    fn test_16() {
        assert_eq!(
            Solution::three_sum_closest(vec![6, 7, 8, 9, 10, 11], 24),
            24
        );
    }

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::three_sum_closest(vec![1, 5, 7, 3, 2, 8, 6, 5, 9], 20),
            20
        );
    }

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::three_sum_closest(vec![-2, -1, 0, 1, 2, 3, 4], 5),
            5
        );
    }

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::three_sum_closest(vec![-3, -2, -1, 0, 1, 2, 3], 0),
            0
        );
    }

    #[test]
    fn test_20() {
        assert_eq!(
            Solution::three_sum_closest(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5], 0),
            0
        );
    }

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10),
            10
        );
    }

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::three_sum_closest(vec![-1, -2, -3, -4, 1, 2, 3, 4], 2),
            2
        );
    }

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 3, 4, -1, -2, -3, -4], -2),
            -2
        );
    }

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::three_sum_closest(vec![9, 11, 12, 13, -10, -11, -12, -13], 24),
            32
        );
    }

    #[test]
    fn test_25() {
        assert_eq!(Solution::three_sum_closest(vec![5, 5, 5, 5, 5, 5], 15), 15);
    }
}

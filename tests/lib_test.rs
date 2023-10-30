#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
            0.0
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2], vec![5, 6, 7]),
            5.5
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2], vec![1, 3, 4]),
            2.5
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]),
            3.5
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6]),
            3.5
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8]),
            4.5
        );
    }

    #[test]
    fn test_10() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![5, 6, 7, 8]),
            4.5
        );
    }

    #[test]
    fn test_11() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 9], vec![4, 5, 6, 7, 8]),
            5.0
        );
    }

    #[test]
    fn test_12() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8]),
            5.0
        );
    }

    #[test]
    fn test_13() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 9], vec![3, 4, 5, 6, 7, 8]),
            5.0
        );
    }

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 9], vec![5, 6, 7, 8]),
            5.0
        );
    }

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![1, 2]),
            1.5
        );
    }

    #[test]
    fn test_16() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]),
            2.5
        );
    }

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]),
            5.5
        );
    }

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]),
            5.5
        );
    }

    #[test]
    fn test_19() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_20() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3]),
            2.0
        );
    }

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 5], vec![3, 4, 6, 7]),
            4.0
        );
    }

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8, 10]),
            5.0
        );
    }

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 4, 6, 8, 10], vec![1, 3, 5, 7]),
            5.0
        );
    }

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![5, 6, 7], vec![1, 2, 3]),
            4.0
        );
    }

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![4, 5, 6, 7], vec![1, 2, 3]),
            4.0
        );
    }

    #[test]
    fn test_26() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![1, 2, 3, 4]),
            2.0
        );
    }

    #[test]
    fn test_27() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![1, 2]),
            2.0
        );
    }

    #[test]
    fn test_28() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![2, 3, 4]),
            3.0
        );
    }

    #[test]
    fn test_29() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 3, 4], vec![1, 2, 3, 4, 5]),
            3.0
        );
    }

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3, 4], vec![1, 2, 3, 4, 5]),
            3.0
        );
    }

    #[test]
    fn test_31() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5]),
            3.0
        );
    }

    #[test]
    fn test_32() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3, 4, 5], vec![1, 2]),
            3.0
        );
    }

    #[test]
    fn test_33() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 4, 5], vec![2, 3]),
            3.0
        );
    }

    #[test]
    fn test_34() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 3], vec![1, 4, 5]),
            3.0
        );
    }

    #[test]
    fn test_35() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]),
            2.5
        );
    }

    #[test]
    fn test_36() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 4], vec![1, 3]),
            2.5
        );
    }

    #[test]
    fn test_37() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1, 2], vec![1, 3, 4]),
            1.5
        );
    }

    #[test]
    fn test_38() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 4], vec![1, 1, 2]),
            1.5
        );
    }

    #[test]
    fn test_39() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_40() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_41() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![100], vec![-100]),
            0.0
        );
    }

    #[test]
    fn test_42() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1, 1, 1], vec![2, 2, 2, 2]),
            1.5
        );
    }

    #[test]
    fn test_43() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![10]),
            6.0
        );
    }

    #[test]
    fn test_44() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![10, 12]),
            7.0
        );
    }

    #[test]
    fn test_45() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11], vec![10, 12]),
            8.0
        );
    }

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11], vec![10, 12, 14]),
            9.0
        );
    }

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13], vec![10, 12, 14]),
            9.5
        );
    }

    #[test]
    fn test_48() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
    }

    #[test]
    fn test_49() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![1]), 1.5);
    }

    #[test]
    fn test_50() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 4, 7, 10, 11], vec![2, 5, 6, 8, 9]),
            6.0
        );
    }
}

#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[inline]
    fn helper(nums: Vec<i32>, mut except: Vec<Vec<i32>>) {
        except.iter_mut().for_each(|el| el.sort());
        except.sort();
        let mut val = Solution::three_sum(nums);
        val.iter_mut().for_each(|el| el.sort());
        val.sort();
        assert_eq!(val, except);
    }

    #[test]
    fn test_1() {
        helper(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
    }

    #[test]
    fn test_2() {
        helper(vec![0, 1, 1], vec![]);
    }

    #[test]
    fn test_3() {
        helper(vec![0, 0, 0], vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_4() {
        helper(vec![-2, 0, 0, 2, 2], vec![vec![-2, 0, 2]]);
    }

    #[test]
    fn test_5() {
        helper(vec![-1, 0, 1, 0], vec![vec![-1, 0, 1]]);
    }

    #[test]
    fn test_6() {
        helper(vec![-3, -1, 0, 2, 4, 5], vec![vec![-3, -1, 4]]);
    }

    #[test]
    fn test_7() {
        helper(
            vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4],
            vec![
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-3, -1, 4],
                vec![-3, 0, 3],
                vec![-3, 1, 2],
                vec![-2, -1, 3],
                vec![-2, 0, 2],
                vec![-1, -1, 2],
                vec![-1, 0, 1],
            ],
        );
    }

    #[test]
    fn test_8() {
        helper(vec![1, 2, 3, 4, 5, 6, 7], vec![]);
    }

    #[test]
    fn test_9() {
        helper(
            vec![-3, -2, -9, 0, 3, 5, 7, 10],
            vec![vec![-3, 0, 3], vec![-3, -2, 5]],
        );
    }

    #[test]
    fn test_10() {
        helper(vec![-9, 1, 3, 5, 7, 9, 10], vec![]);
    }

    #[test]
    fn test_11() {
        helper(vec![-10, -10, 20, 30], vec![vec![-10, -10, 20]]);
    }

    #[test]
    fn test_12() {
        helper(
            vec![5, -3, -4, -7, -7, -3, -6, -5, -7, 2, -3, -7],
            vec![vec![-7, 2, 5]],
        );
    }

    #[test]
    fn test_13() {
        helper(
            vec![-3, 0, 1, 2, -1, 1, -2],
            vec![
                vec![-3, 1, 2],
                vec![-2, 0, 2],
                vec![-2, 1, 1],
                vec![-1, 0, 1],
            ],
        );
    }

    #[test]
    fn test_14() {
        helper(vec![1, -1, -1, 0], vec![vec![-1, 0, 1]]);
    }

    #[test]
    fn test_15() {
        helper(
            vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4],
            vec![
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-3, -1, 4],
                vec![-3, 0, 3],
                vec![-3, 1, 2],
                vec![-2, -1, 3],
                vec![-2, 0, 2],
                vec![-1, -1, 2],
                vec![-1, 0, 1],
            ],
        );
    }

    #[test]
    fn test_16() {
        helper(vec![1, 1, -2], vec![vec![-2, 1, 1]]);
    }

    #[test]
    fn test_17() {
        helper(vec![4, 0, -1, -2, -1, -3], vec![vec![-3, -1, 4]]);
    }

    #[test]
    fn test_18() {
        helper(vec![0, 0, 0, 0], vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_19() {
        helper(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
    }

    #[test]
    fn test_20() {
        helper(vec![0, 0, 0], vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_21() {
        helper(vec![2, 2, 2, 2], vec![]);
    }

    #[test]
    fn test_22() {
        helper(vec![1, 0, -1, 0, 0, 2], vec![vec![-1, 0, 1], vec![0, 0, 0]]);
    }

    #[test]
    fn test_23() {
        helper(vec![-2, 0, 1, 1, 2], vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
    }

    #[test]
    fn test_24() {
        helper(
            vec![1, 0, -1, 0, -2, 2],
            vec![vec![-2, 0, 2], vec![-1, 0, 1]],
        );
    }

    #[test]
    fn test_25() {
        helper(vec![5, 5, 3, 5, 1, -5, 1, -2], vec![vec![1, 1, -2]]);
    }

    #[test]
    fn test_26() {
        helper(std::iter::repeat(0).take(3000).collect(), vec![vec![0, 0, 0]]);
    }
}

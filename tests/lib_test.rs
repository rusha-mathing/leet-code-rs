#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    #[inline]
    fn helper(mut result: Vec<Vec<i32>>, mut except: Vec<Vec<i32>>) {
        except.iter_mut().for_each(|el| el.sort());
        except.sort();
        result.iter_mut().for_each(|el| el.sort());
        result.sort();
        assert_eq!(result, except);
    }

    #[test]
    fn test_1() {
        helper(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        )
    }

    #[test]
    fn test_2() {
        helper(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]],
        )
    }

    #[test]
    fn test_3() {
        helper(
            Solution::four_sum(vec![1, 2, 3, 4, 5, 6], 10),
            vec![vec![1, 2, 3, 4]],
        )
    }

    #[test]
    fn test_4() {
        helper(
            Solution::four_sum(vec![-1, 0, 1, 2, -1, -4], -1),
            vec![vec![-4, 0, 1, 2], vec![-1, -1, 0, 1]],
        )
    }

    #[test]
    fn test_5() {
        helper(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 2),
            vec![vec![-3, -1, 2, 4]],
        )
    }

    #[test]
    fn test_6() {
        helper(
            Solution::four_sum(vec![1, 2, 2, 3, 4, 4, 5], 12),
            vec![
                vec![1, 2, 4, 5],
                vec![2, 2, 4, 4],
                vec![1, 3, 4, 4],
                vec![2, 2, 3, 5],
            ],
        )
    }

    #[test]
    fn test_7() {
        helper(
            Solution::four_sum(vec![0, 0, 0, 0], 0),
            vec![vec![0, 0, 0, 0]],
        )
    }

    #[test]
    fn test_8() {
        helper(
            Solution::four_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 20),
            vec![
                vec![1, 2, 8, 9],
                vec![1, 3, 7, 9],
                vec![1, 4, 6, 9],
                vec![1, 4, 7, 8],
                vec![1, 5, 6, 8],
                vec![2, 3, 6, 9],
                vec![2, 3, 7, 8],
                vec![2, 4, 5, 9],
                vec![2, 4, 6, 8],
                vec![2, 5, 6, 7],
                vec![3, 4, 5, 8],
                vec![3, 4, 6, 7],
            ],
        )
    }

    #[test]
    fn test_9() {
        helper(
            Solution::four_sum(vec![-1, -1, -1, 0, 1, 1, 1, 1], 2),
            vec![vec![-1, 1, 1, 1]],
        )
    }

    #[test]
    fn test_10() {
        helper(
            Solution::four_sum(vec![-2, -1, 2, 0], -1),
            vec![vec![-2, -1, 0, 2]],
        )
    }

    #[test]
    fn test_11() {
        helper(
            Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]],
        )
    }

    #[test]
    fn test_12() {
        helper(
            Solution::four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0),
            vec![
                vec![-3, -2, 2, 3],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1],
            ],
        )
    }

    #[test]
    fn test_13() {
        helper(
            Solution::four_sum(vec![-1, 0, 1, 2, -1, -4], -1),
            vec![vec![-4, 0, 1, 2], vec![-1, -1, 0, 1]],
        )
    }

    #[test]
    fn test_14() {
        helper(
            Solution::four_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4], -1),
            vec![
                vec![-4, -3, 2, 4],
                vec![-4, -2, 1, 4],
                vec![-4, -2, 2, 3],
                vec![-4, -1, 0, 4],
                vec![-4, -1, 1, 3],
                vec![-4, 0, 0, 3],
                vec![-4, 0, 1, 2],
                vec![-3, -2, 0, 4],
                vec![-3, -2, 1, 3],
                vec![-3, -1, -1, 4],
                vec![-3, -1, 0, 3],
                vec![-3, -1, 1, 2],
                vec![-3, 0, 0, 2],
                vec![-2, -1, -1, 3],
                vec![-2, -1, 0, 2],
                vec![-2, 0, 0, 1],
                vec![-1, -1, 0, 1],
            ],
        )
    }

    #[test]
    fn test_15() {
        helper(
            Solution::four_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4, 4, 5, 6, 7], 3),
            vec![
                vec![-4, -3, 3, 7],
                vec![-4, -3, 4, 6],
                vec![-4, -2, 2, 7],
                vec![-4, -2, 3, 6],
                vec![-4, -2, 4, 5],
                vec![-4, -1, 1, 7],
                vec![-4, -1, 2, 6],
                vec![-4, -1, 3, 5],
                vec![-4, -1, 4, 4],
                vec![-4, 0, 0, 7],
                vec![-4, 0, 1, 6],
                vec![-4, 0, 2, 5],
                vec![-4, 0, 3, 4],
                vec![-4, 1, 2, 4],
                vec![-3, -2, 1, 7],
                vec![-3, -2, 2, 6],
                vec![-3, -2, 3, 5],
                vec![-3, -2, 4, 4],
                vec![-3, -1, 0, 7],
                vec![-3, -1, 1, 6],
                vec![-3, -1, 2, 5],
                vec![-3, -1, 3, 4],
                vec![-3, 0, 0, 6],
                vec![-3, 0, 1, 5],
                vec![-3, 0, 2, 4],
                vec![-3, 1, 2, 3],
                vec![-2, -1, -1, 7],
                vec![-2, -1, 0, 6],
                vec![-2, -1, 1, 5],
                vec![-2, -1, 2, 4],
                vec![-2, 0, 0, 5],
                vec![-2, 0, 1, 4],
                vec![-2, 0, 2, 3],
                vec![-1, -1, 0, 5],
                vec![-1, -1, 1, 4],
                vec![-1, -1, 2, 3],
                vec![-1, 0, 0, 4],
                vec![-1, 0, 1, 3],
                vec![0, 0, 1, 2],
            ],
        )
    }

    #[test]
    fn test_16() {
        helper(
            Solution::four_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4, 4, 5, 6, 7], 6),
            vec![
                vec![-4, -3, 6, 7],
                vec![-4, -2, 5, 7],
                vec![-4, -1, 4, 7],
                vec![-4, -1, 5, 6],
                vec![-4, 0, 3, 7],
                vec![-4, 0, 4, 6],
                vec![-4, 1, 2, 7],
                vec![-4, 1, 3, 6],
                vec![-4, 1, 4, 5],
                vec![-4, 2, 3, 5],
                vec![-4, 2, 4, 4],
                vec![-3, -2, 4, 7],
                vec![-3, -2, 5, 6],
                vec![-3, -1, 3, 7],
                vec![-3, -1, 4, 6],
                vec![-3, 0, 2, 7],
                vec![-3, 0, 3, 6],
                vec![-3, 0, 4, 5],
                vec![-3, 1, 2, 6],
                vec![-3, 1, 3, 5],
                vec![-3, 1, 4, 4],
                vec![-3, 2, 3, 4],
                vec![-2, -1, 2, 7],
                vec![-2, -1, 3, 6],
                vec![-2, -1, 4, 5],
                vec![-2, 0, 1, 7],
                vec![-2, 0, 2, 6],
                vec![-2, 0, 3, 5],
                vec![-2, 0, 4, 4],
                vec![-2, 1, 2, 5],
                vec![-2, 1, 3, 4],
                vec![-1, -1, 1, 7],
                vec![-1, -1, 2, 6],
                vec![-1, -1, 3, 5],
                vec![-1, -1, 4, 4],
                vec![-1, 0, 0, 7],
                vec![-1, 0, 1, 6],
                vec![-1, 0, 2, 5],
                vec![-1, 0, 3, 4],
                vec![-1, 1, 2, 4],
                vec![0, 0, 1, 5],
                vec![0, 0, 2, 4],
                vec![0, 1, 2, 3],
            ],
        )
    }

    #[test]
    fn test_17() {
        helper(
            Solution::four_sum(vec![-1, -5, -5, -3, 2, 5, 0, 4], -7),
            vec![vec![-5, -5, -1, 4], vec![-5, -3, -1, 2]],
        )
    }

    #[test]
    fn test_18() {
        helper(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        )
    }

    #[test]
    fn test_19() {
        helper(
            Solution::four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0),
            vec![
                vec![-3, -2, 2, 3],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1],
            ],
        )
    }

    #[test]
    fn test_20() {
        helper(
            Solution::four_sum(vec![0, 0, 0, 0], 0),
            vec![vec![0, 0, 0, 0]],
        )
    }

    #[test]
    fn test_21() {
        helper(Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296), vec![])
    }
}

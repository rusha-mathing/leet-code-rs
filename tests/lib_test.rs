#[cfg(test)]
mod tests {
    use leet_code_rs::{list::vec_to_list_node, Solution};

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![2, 4, 3]),
                vec_to_list_node(vec![5, 6, 4])
            ),
            vec_to_list_node(vec![7, 0, 8])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(vec_to_list_node(vec![0]), vec_to_list_node(vec![0])),
            vec_to_list_node(vec![0])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![9, 9, 9, 9, 9, 9, 9]),
                vec_to_list_node(vec![9, 9, 9, 9])
            ),
            vec_to_list_node(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::add_two_numbers(vec_to_list_node(vec![8]), vec_to_list_node(vec![7])),
            vec_to_list_node(vec![5, 1])
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::add_two_numbers(vec_to_list_node(vec![1]), vec_to_list_node(vec![9, 9, 9])),
            vec_to_list_node(vec![0, 0, 0, 1])
        )
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![1, 1, 1]),
                vec_to_list_node(vec![9, 9])
            ),
            vec_to_list_node(vec![0, 1, 2]),
        )
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![1, 2, 3, 4, 5]),
                vec_to_list_node(vec![5, 4, 3, 2, 1])
            ),
            vec_to_list_node(vec![6, 6, 6, 6, 6])
        )
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![1]),
                vec_to_list_node(vec![9, 9, 9, 8])
            ),
            vec_to_list_node(vec![0, 0, 0, 9])
        )
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![0, 1, 2, 3, 4]),
                vec_to_list_node(vec![0, 5, 6, 7, 8, 9])
            ),
            vec_to_list_node(vec![0, 6, 8, 0, 3, 0, 1])
        )
    }

    #[test]
    fn test_10() {
        assert_eq!(
            Solution::add_two_numbers(vec_to_list_node(vec![1]), vec_to_list_node(vec![9, 8]),),
            vec_to_list_node(vec![0, 9])
        )
    }

    #[test]
    fn test_11() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![3, 4, 5, 9]),
                vec_to_list_node(vec![9, 7, 6])
            ),
            vec_to_list_node(vec![2, 2, 2, 0, 1])
        )
    }

    #[test]
    fn test_12() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![7, 8, 9]),
                vec_to_list_node(vec![1, 0, 1])
            ),
            vec_to_list_node(vec![8, 8, 0, 1])
        )
    }
}

#[cfg(test)]
mod tests {
    use leet_code_rs::{list::vec_to_list_node, Solution};

    fn helper(list1: Vec<i32>, list2: Vec<i32>, except: Vec<i32>) {
        assert_eq!(
            Solution::merge_two_lists(vec_to_list_node(list1), vec_to_list_node(list2)),
            vec_to_list_node(except)
        );
    }

    #[test]
    fn test_1() {
        helper(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4])
    }

    #[test]
    fn test_2() {
        helper(vec![], vec![1], vec![1]);
        helper(vec![1], vec![], vec![1]);
    }

    #[test]
    fn test_3() {
        helper(vec![], vec![], vec![])
    }

    #[test]
    fn test_4() {
        helper(vec![1, 1, 2], vec![1, 4, 5, 6], vec![1, 1, 1, 2, 4, 5, 6]);
    }
}

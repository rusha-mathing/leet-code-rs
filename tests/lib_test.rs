#[cfg(test)]
mod tests {
    use leet_code_rs::{list::vec_to_list_node, Solution};

    fn helper(vec: Vec<i32>, n: i32, except: Vec<i32>) {
        assert_eq!(
            Solution::remove_nth_from_end(vec_to_list_node(vec), n),
            vec_to_list_node(except)
        );
    }

    #[test]
    fn test_1() {
        helper(vec![1, 2, 3], 1, vec![1, 2])
    }

    #[test]
    fn test_2() {
        helper(vec![1, 2, 3], 2, vec![1, 3])
    }

    #[test]
    fn test_3() {
        helper(vec![1, 2, 3], 3, vec![2, 3])
    }

    #[test]
    fn test_4() {
        helper(vec![1], 1, vec![])
    }

    #[test]
    fn test_5() {
        helper(vec![1, 2], 1, vec![1])
    }

    #[test]
    fn test_6() {
        helper(vec![1, 2], 2, vec![2])
    }

    #[test]
    fn test_7() {
        helper((1..=30).collect(), 1, (1..=29).collect())
    }

    #[test]
    fn test_8() {
        helper((1..=30).collect(), 7, (1..=23).chain(25..=30).collect())
    }
}

#[cfg(test)]
mod tests {
    use leet_code_rs::{list::*, Solution};

    fn helper(vec: Vec<Vec<i32>>, except: Vec<i32>) {
        let result = Solution::merge_k_lists(vec.into_iter().map(vec_to_list_node).collect());
        let except = vec_to_list_node(except);
        assert_eq!(result, except);
    }

    #[test]
    fn test_1() {
        helper(vec![
            vec![1, 4, 5],
            vec![1, 3, 4],
            vec![2, 6]
        ], vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_2() {
        helper(vec![vec![]], vec![])
    }

    #[test]
    fn test_3() {
        helper(vec![], vec![])
    }

    #[test]
    fn test_4() {
        helper(vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ], vec![1, 1, 1, 2, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn test_5() {
        helper(vec![vec![1; 10]; 10], vec![1; 100]);
    }    

    #[test]
    fn test_6() {
        helper(vec![vec![]; 100], vec![]);
    }
}

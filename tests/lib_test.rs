
#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn vec_eq(mut v1: Vec<i32>, mut v2: Vec<i32>) -> bool {
        v1.sort(); v2.sort();
        v1 == v2
    }

    #[test]
    fn test_1() {
        assert!(vec_eq(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]));
    }

    #[test]
    fn test_2() {
        assert!(vec_eq(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]));
    }

    #[test]
    fn test_3() {
        assert!(vec_eq(Solution::two_sum(vec![3, 3], 6), vec![0, 1]))
    }

    #[test]
    fn test_4() {
        assert!(vec_eq(Solution::two_sum(vec![1, 2, 3, 4, 5], 9), vec![3, 4]));
    }
}
#[cfg(test)]
mod tests {
    use leet_code_rs::list::*;

    #[test]
    fn test_create_node() {
        let node = ListNode::new(1);
        assert_eq!(node.val, 1);
        assert_eq!(node.next, None);
    }

    #[test]
    fn test_empty_vec_to_node() {
        let node = vec_to_list_node(vec![]);
        assert_eq!(node, None);
    }

    #[test]
    fn test_one_element_to_node() {
        for i in 0..=9 {
            let node = vec_to_list_node(vec![i]);
            assert_eq!(node, Some(Box::new(ListNode::new(i))));
        }
    }

    #[test]
    fn test_two_element_to_node() {
        let node = vec_to_list_node(vec![1, 2]);

        assert_eq!(
            node,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))
        )
    }

    #[test]
    fn test_three_element_to_node() {
        let node = vec_to_list_node(vec![1, 2, 3]);
        assert_eq!(
            node,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))
        )
    }

    #[test]
    fn test_empty_len() {
        let node = vec_to_list_node(vec![]);
        assert_eq!(list_len(&node), 0);
    }

    #[test]
    fn test_one_len() {
        let node = vec_to_list_node(vec![1]);
        assert_eq!(list_len(&node), 1);
    }

    #[test]
    fn test_many_len() {
        let node = vec_to_list_node((1..=100).collect());
        assert_eq!(list_len(&node), 100);
    }
}

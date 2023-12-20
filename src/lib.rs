pub mod list;

use list::ListNode;
pub struct Solution;
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_two_lists(mut list1: Node, mut list2: Node) -> Node {
        let mut result = None;
        let mut tail = &mut result;
        while let Some(val) = get_min(&mut list1, &mut list2) {
            tail = &mut tail.insert(val).next;
        }
        result
    }
}

fn get_min(list1: &mut Node, list2: &mut Node) -> Node {
    match (list1.take(), list2.take()) {
        (None, None) => None,
        (None, Some(node)) | (Some(node), None) => {
            (*list1, *list2) = (None, None);
            Some(node)
        }
        (Some(i), Some(j)) => Some(Box::new(ListNode::new(if i.val < j.val {
            (*list1, *list2) = (i.next, Some(j));
            i.val
        } else {
            (*list1, *list2) = (Some(i), j.next);
            j.val
        }))),
    }
}

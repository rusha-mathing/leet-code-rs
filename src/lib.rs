/// The structure definition for this module can be found in list.rs
pub mod list;
use list::ListNode;
pub struct Solution();
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(mut l1: Node, mut l2: Node) -> Node {
        let mut head = None;
        let mut tail = &mut head;
        let mut remainder = 0;
        loop {
            let val = match (l1, l2) {
                (None, None) => {
                    if remainder != 0 {
                        *tail = Some(Box::new(ListNode::new(remainder)));
                    }
                    return head;
                }
                (None, Some(node)) | (Some(node), None) => {
                    (l1, l2) = (node.next, None);
                    node.val
                }
                (Some(left), Some(right)) => {
                    (l1, l2) = (left.next, right.next);
                    left.val + right.val
                }
            } + remainder;
            remainder = val / 10;
            tail = &mut tail.insert(Box::new(ListNode::new(val % 10))).next;
        }
    }
}

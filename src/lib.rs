/// The structure definition for this module can be found in list.rs
pub mod list;

use list::ListNode;

use crate::list::list_len;
pub struct Solution;
type Node = Option<Box<ListNode>>;

fn helper(val: i32) -> Box<ListNode> {
    Box::new(ListNode::new(val))
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Node, n: i32) -> Node {
        let n = n as usize;
        let mut len = list_len(&head);
        let mut result = None;
        let mut tail = &mut result;
        while len > n  {
            let temp = head.expect("Except that n is not great that len!");
            tail = &mut tail.insert(helper(temp.val)).next;
            head = temp.next;
            len -= 1;
        }
        *tail = head.expect("Except n >= 1").next;
        result
    }
}

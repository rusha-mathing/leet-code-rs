/// The structure definition for this module can be found in list.rs
pub mod list;

use list::ListNode;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub struct Solution;
type Node = Option<Box<ListNode>>;

fn helper(val: i32) -> Box<ListNode> {
    Box::new(ListNode::new(val))
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Node>) -> Node {
        let mut heap = lists.into_iter()
            .filter(Node::is_some)
            .map(Reverse)
            .collect::<BinaryHeap<_>>();
        let mut head: Node = None;
        let mut tail = &mut head;
        while let Some(Reverse(Some(node))) = heap.pop() {
            tail = &mut tail.insert(helper(node.val)).next;
            if node.next.is_some() {
                heap.push(Reverse(node.next));
            }
        }
        head
    }
}

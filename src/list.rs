pub type Node = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Node,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vec_to_list_node(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let nums = nums.into_iter().map(ListNode::new).map(Box::new);
    let mut head: Node = None;
    nums.fold(&mut head, |mut acc, node| {
        acc = &mut acc.insert(node).next;
        acc
    });
    head
}

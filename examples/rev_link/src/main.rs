#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur_head = head;
    let mut new_head = None::<Box<ListNode>>;

    while let Some(mut cur_node) = cur_head {
        cur_head = cur_node.next;
        cur_node.next = new_head;
        new_head = Some(cur_node);
    }
    new_head
}

fn main() {
    let mut head_node = Box::new(ListNode::new(1));
    let mut sec_node = Box::new(ListNode::new(2));
    let mut third_node = Box::new(ListNode::new(3));

    sec_node.next = Some(third_node);
    head_node.next = Some(sec_node);

    let x = reverse_list(Some(head_node.clone()));
    println!("{:?}", x)
}

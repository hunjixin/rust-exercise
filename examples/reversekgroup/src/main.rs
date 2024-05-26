use std::ptr::NonNull;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<NonNull<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn push_back(&mut self, val: i32) {
        let mut cur = self;
        unsafe {
            loop {
                if cur.next.is_none() {
                    cur.next = NonNull::new(Box::leak(Box::new(ListNode::new(val))));
                    break;
                }
                cur = cur.next.unwrap().as_mut();
            }
        }
    }
}

fn reverse_list(head: Option<NonNull<ListNode>>) -> Option<NonNull<ListNode>> {
    let mut cur_head = head;
    let mut new_head = None::<NonNull<ListNode>>;

    unsafe {
        while let Some(mut cur_node) = cur_head {
            cur_head = cur_node.as_ref().next;
            cur_node.as_mut().next = new_head;
            new_head = Some(cur_node);
        }
    }
    new_head
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn reverse_k_group(
    head: Option<NonNull<ListNode>>,
    k: i32,
) -> Option<NonNull<ListNode>> {
    let mut cur_head = head;
    let mut last_tail = None::<NonNull<ListNode>>;
    let mut new_head = head;
    let mut count = 0;

    loop {
        let start_k_head = cur_head; //keep start head
        while let Some(mut cur_node) = cur_head {
            count += 1;
            if count % k == 0 {
                //find k group
                let next_head = cur_node.as_ref().next; //keep next group head
                cur_node.as_mut().next = None; //split k group  this subgroup from (start_k_head..cur_node)

                let k_list = reverse_list(start_k_head); //reverse subgroup
                if count == k {
                    //assigne newhead for first k group
                    new_head = k_list;
                } else {
                    last_tail.as_mut().unwrap().as_mut().next = k_list; //join current group to previous group
                }

                last_tail = start_k_head;

                cur_head = next_head; //assign for next round
                break;
            }

            cur_head = cur_node.as_ref().next;
        }

        if cur_head.is_none() {
            last_tail.as_mut().unwrap().as_mut().next = start_k_head;
            break;
        }
    }

    new_head
}

fn println(head: Option<NonNull<ListNode>>) {
    let mut cur = head;
    unsafe {
        while let Some(v) = cur {
            print!("{} ", v.as_ref().val);
            cur = v.as_ref().next;
        }
    }
    println!();
}

fn main() {
    unsafe {
        let mut head_node = ListNode::new(1);
        head_node.push_back(2);
        head_node.push_back(3);
        head_node.push_back(4);
        head_node.push_back(5);

        let head_node = NonNull::new(Box::leak(Box::new(head_node)));
        println(head_node);

        let rev_k_node = reverse_k_group(head_node, 3);
        println(rev_k_node);
    }
}

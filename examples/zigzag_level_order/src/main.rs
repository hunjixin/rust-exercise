// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;
use std::rc::Rc;
use std::vec;

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut odd_dep = VecDeque::new();
    let mut evn_dep = VecDeque::from([root.unwrap()]);

    let mut result = vec![];
    let mut level = 0;
    while !(odd_dep.is_empty() && evn_dep.is_empty()) {
        let mut level_vals = vec![];
        if level % 2 == 0 {
            while !evn_dep.is_empty() {
                let node = evn_dep.pop_front();
                if let Some(node) = node {
                    level_vals.push(node.borrow().val);
                    if let Some(left) = &node.borrow().left {
                        odd_dep.push_back(Rc::clone(left));
                    }

                    if let Some(leftright) = &node.borrow().right {
                        odd_dep.push_back(Rc::clone(leftright));
                    }
                }
            }
        } else {
            while !odd_dep.is_empty() {
                let node = odd_dep.pop_back();
                if let Some(node) = node {
                    level_vals.push(node.borrow().val);

                    if let Some(right) = &node.borrow().right {
                        evn_dep.push_front(Rc::clone(right));
                    }

                    if let Some(left) = &node.borrow().left {
                        evn_dep.push_front(Rc::clone(left));
                    }
                }
            }
        }
        result.push(level_vals);
        level += 1;
    }

    result
}

fn zigzag_level_order_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut dep = VecDeque::from([root.unwrap()]);
    let mut result = vec![];
    let mut reverse = false;

    while !dep.is_empty() {
        let mut level_vals = vec![];
        for _ in 0..dep.len() {
            if let Some(node) = dep.pop_front() {
                level_vals.push(node.borrow().val);
                if let Some(left) = &node.borrow().left {
                    dep.push_back(Rc::clone(left));
                }

                if let Some(leftright) = &node.borrow().right {
                    dep.push_back(Rc::clone(leftright));
                }
            }
        }
        if reverse {
            level_vals.reverse();
        }
        reverse = !reverse;
        result.push(level_vals);
    }

    result
}

fn main() {
    {
        //[[3], [20, 9], [7, 15]]
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);

        let mut node20 = TreeNode::new(20);
        let mut node9 = TreeNode::new(9);

        let mut node3 = TreeNode::new(3);

        node20.left = Some(Rc::new(RefCell::new(node15)));
        node9.left = Some(Rc::new(RefCell::new(node7)));

        node3.left = Some(Rc::new(RefCell::new(node9)));
        node3.right = Some(Rc::new(RefCell::new(node20)));

        let root = Some(Rc::new(RefCell::new(node3)));

        let result = zigzag_level_order_v2(root);

        println!("{:?}", result);
    }

    {
        //[[3], [20, 9], [15, 7]]
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);

        let mut node20 = TreeNode::new(20);
        let mut node9 = TreeNode::new(9);

        let mut node3 = TreeNode::new(3);

        node20.left = Some(Rc::new(RefCell::new(node15)));
        node20.right = Some(Rc::new(RefCell::new(node7)));

        node3.left = Some(Rc::new(RefCell::new(node9)));
        node3.right = Some(Rc::new(RefCell::new(node20)));

        let root = Some(Rc::new(RefCell::new(node3)));

        let result = zigzag_level_order_v2(root);

        println!("{:?}", result);
    }
}

use std::cell::RefCell;
use std::cmp;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            let left_depth = max_depth(node_ref.left.take());
            let right_depth = max_depth(node_ref.right.take());
            1 + cmp::max(left_depth, right_depth)
        }
    }
}

#[allow(dead_code)]
fn max_depth_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut depth = 0;
    while queue.len() > 0 {
        let mut level_node_count = queue.len();
        depth += 1;
        while level_node_count > 0 {
            let node = queue.pop_front().unwrap();
            let mut borrowed = node.borrow_mut();
            if let Some(left) = borrowed.left.take() {
                queue.push_back(left);
            }
            if let Some(right) = borrowed.right.take() {
                queue.push_back(right);
            }
            level_node_count -= 1;
        }
    }
    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        run_tests(max_depth);
    }

    #[test]
    fn test_max_depth_iterative() {
        run_tests(max_depth_iterative);
    }

    fn run_tests<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> i32>(fun: F) {
        assert_eq!(fun(make_tree(1, 2, 3)), 2);
        assert_eq!(fun(Some(Rc::new(RefCell::new(TreeNode::new(50))))), 1);
    }

    fn make_tree(middle: i32, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(left))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(right))));
        let middle = TreeNode {
            val: middle,
            left,
            right,
        };
        Some(Rc::new(RefCell::new(middle)))
    }
}

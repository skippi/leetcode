use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
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
fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(node) => {
            let mut node = node.borrow_mut();
            equal_tree(node.left.take(), flip_tree(node.right.take()))
        }
    }
}

fn equal_tree(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let mut a = x.borrow_mut();
            let mut b = y.borrow_mut();
            a.val == b.val
                && equal_tree(a.left.take(), b.left.take())
                && equal_tree(a.right.take(), b.right.take())
        }
        _ => false,
    }
}

fn flip_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => unsafe {
            let mut borrowed = node.as_ptr();
            let flipped_left = flip_tree((*borrowed).left.take());
            let flipped_right = flip_tree((*borrowed).right.take());
            (*borrowed).left = flipped_right;
            (*borrowed).right = flipped_left;
            Some(node)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    macro_rules! tree {
        [ $($x:expr),*] => { make_tree(vec![$($x,)*]) };
    }

    fn make_tree(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result: Option<Rc<RefCell<TreeNode>>> = None;
        let mut queue = VecDeque::<*mut Option<Rc<RefCell<TreeNode>>>>::new();
        queue.push_back(&mut result);
        for n in data {
            if queue.is_empty() {
                break;
            }
            let node = queue.pop_front().unwrap();
            if let Some(n) = n {
                unsafe {
                    let leaf = Rc::new(RefCell::new(TreeNode::new(n)));
                    queue.push_back(&mut (*leaf.as_ptr()).left);
                    queue.push_back(&mut (*leaf.as_ptr()).right);
                    *node = Some(leaf)
                }
            }
        }
        result
    }

    #[test]
    fn test_is_symmetric() {
        assert!(is_symmetric(tree![]));
        assert!(!is_symmetric(tree![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3)
        ]));
        assert!(is_symmetric(tree![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3)
        ]));
        assert!(!is_symmetric(tree![Some(1), Some(2), Some(3)]))
    }
}

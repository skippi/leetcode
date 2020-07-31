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
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(x), Some(y)) if x.borrow().val == y.borrow().val => {
            let mut x = x.borrow_mut();
            let mut y = y.borrow_mut();
            is_same_tree(x.left.take(), y.left.take())
                && is_same_tree(x.right.take(), y.right.take())
        }
        (None, None) => true,
        _ => false,
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
    fn test_is_same_tree() {
        assert!(is_same_tree(tree![], tree![]));
        assert!(!is_same_tree(tree![], tree![Some(0)]));
        assert!(is_same_tree(tree![Some(0)], tree![Some(0)]));
        assert!(!is_same_tree(tree![Some(0), Some(1)], tree![Some(0)]));
        assert!(is_same_tree(
            tree![Some(0), Some(1)],
            tree![Some(0), Some(1)]
        ));
    }
}

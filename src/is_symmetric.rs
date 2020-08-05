use std::cell::RefCell;
use std::collections::VecDeque;
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
fn is_symmetric_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue: VecDeque<Option<&Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root.as_ref());
    queue.push_back(root.as_ref());
    while queue.len() >= 2 {
        let a = queue.pop_front().unwrap();
        let b = queue.pop_front().unwrap();
        match (a, b) {
            (Some(x), Some(y)) => unsafe {
                let a = x.as_ptr();
                let b = y.as_ptr();
                if (*a).val != (*b).val {
                    return false;
                }
                queue.push_back((*a).left.as_ref());
                queue.push_back((*b).right.as_ref());
                queue.push_back((*a).right.as_ref());
                queue.push_back((*b).left.as_ref());
            },
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }
    }
    true
}

#[allow(dead_code)]
fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_mirror(root.as_ref(), root.as_ref())
}

fn is_mirror(a: Option<&Rc<RefCell<TreeNode>>>, b: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let a = x.borrow();
            let b = y.borrow();
            a.val == b.val
                && is_mirror(a.left.as_ref(), b.right.as_ref())
                && is_mirror(a.right.as_ref(), b.left.as_ref())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_is_symmetric_iterative() {
        run_tests(is_symmetric_iterative);
    }

    #[test]
    fn test_is_symmetric() {
        run_tests(is_symmetric);
    }

    fn run_tests<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> bool>(fun: F) {
        assert!(fun(tree![]));
        assert!(!fun(tree![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3)
        ]));
        assert!(fun(tree![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3)
        ]));
        assert!(!fun(tree![Some(1), Some(2), Some(3)]))
    }
}

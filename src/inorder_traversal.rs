use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[cfg(test)]
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
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        None => vec![],
        Some(rc) => {
            let mut node = rc.borrow_mut();
            inorder_traversal(node.left.take())
                .into_iter()
                .chain(iter::once(node.val))
                .chain(inorder_traversal(node.right.take()).into_iter())
                .collect()
        }
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
    fn test_inorder_traversal() {
        assert_eq!(inorder_traversal(tree![Some(1)]), vec![1]);
        assert_eq!(inorder_traversal(tree![]), vec![]);
        assert_eq!(inorder_traversal(tree![Some(3)]), vec![3]);
        assert_eq!(inorder_traversal(tree![Some(1), Some(2)]), vec![2, 1]);
        assert_eq!(
            inorder_traversal(tree![Some(1), None, Some(2), Some(3)]),
            vec![1, 3, 2]
        );
    }
}

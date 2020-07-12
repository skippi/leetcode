use std::cell::RefCell;
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
fn merge_trees(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match (t1, t2) {
        (None, t2) => t2,
        (t1, None) => t1,
        (Some(t1), Some(t2)) => {
            let mut t1 = t1.borrow_mut();
            let mut t2 = t2.borrow_mut();
            let node = TreeNode {
                val: t1.val + t2.val,
                left: merge_trees(t1.left.take(), t2.left.take()),
                right: merge_trees(t1.right.take(), t2.right.take()),
            };
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_trees() {
        assert_eq!(
            merge_trees(make_tree(1, 3, 2), make_tree(3, 3, 3)),
            make_tree(4, 6, 5)
        );
        assert_eq!(
            merge_trees(
                Some(Rc::new(RefCell::new(TreeNode::new(50)))),
                make_tree(20, 3, 3)
            ),
            make_tree(70, 3, 3)
        );
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

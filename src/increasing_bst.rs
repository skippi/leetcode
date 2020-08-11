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
fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut result = None;
    dfs(root.as_ref(), &mut result);
    result
}

fn dfs(tree: Option<&Rc<RefCell<TreeNode>>>, result: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(x) = tree {
        let node = x.borrow();
        dfs(node.right.as_ref(), result);
        *result = Some(Rc::new(RefCell::new(TreeNode {
            val: node.val,
            left: None,
            right: result.take(),
        })));
        dfs(node.left.as_ref(), result);
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
    fn test_increasing_bst() {
        assert_eq!(increasing_bst(tree![None]), None);
        assert_eq!(increasing_bst(tree![Some(1)]), tree![Some(1)]);
        assert_eq!(
            increasing_bst(tree![Some(2), Some(1)]),
            tree![Some(1), None, Some(2)]
        );
        assert_eq!(
            increasing_bst(tree![Some(2), Some(1), Some(3), Some(0)]),
            tree![Some(0), None, Some(1), None, Some(2), None, Some(3)]
        );
    }
}

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

/// Think about how the recursive implementation works from a stack perspective.  That
/// implementation "folds" out to the left until it reaches the left most node, after which the
/// implementation retracts towards the right to iterate the tree in-order.
///
/// Keeping this in mind, we can mimic the same implementation using just a stack.  This is done by
/// pushing left nodes continuously until the leftmost node is reached.  We can then iterate these
/// nodes by popping the stack to mimic the recursive method.
///
/// However, if one of these popped nodes has a right branch, then we need to "restart" the
/// algorithm from the beginning. This is because, like the recursive solution, we need to "fold"
/// out all the leftward nodes. Think about it: If we only iterate left nodes, we'll never reach
/// the branches that originate from a right branch. :^)
#[allow(dead_code)]
fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut stack: Vec<Option<*mut TreeNode>> = vec![];
    stack.push(root.as_ref().map(|o| o.as_ptr()));
    while let Some(top) = stack.pop() {
        let mut current = top;
        while let Some(node) = current {
            stack.push(current);
            unsafe { current = (*node).left.as_ref().map(|o| o.as_ptr()) }
        }
        while let Some(elem) = stack.pop() {
            unsafe {
                if let Some(node1) = elem {
                    result.push((*node1).val);
                    if (*node1).right.is_some() {
                        stack.push((*node1).right.as_ref().map(|o| o.as_ptr()));
                        break;
                    }
                }
            }
        }
    }
    result
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
        run_tests(inorder_traversal);
    }

    #[test]
    fn test_inorder_traversal_iterative() {
        run_tests(inorder_traversal_iterative);
    }

    fn run_tests<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>>(fun: F) {
        assert_eq!(fun(tree![Some(1)]), vec![1]);
        assert_eq!(fun(tree![]), vec![]);
        assert_eq!(fun(tree![Some(3)]), vec![3]);
        assert_eq!(fun(tree![Some(1), Some(2)]), vec![2, 1]);
        assert_eq!(fun(tree![Some(1), None, Some(2), Some(3)]), vec![1, 3, 2]);
    }
}

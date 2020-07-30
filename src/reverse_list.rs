#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;
    while head.is_some() {
        let val = head.as_ref().map(|n| n.val).unwrap();
        result = Some(Box::new(ListNode { val, next: result }));
        head = head.and_then(|node| node.next);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! list {
        [$($x:expr), *] => {
            {
                make_list(&[$($x,)*])
            }
        }
    }

    fn make_list(data: &[i32]) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        for &n in data.iter().rev() {
            result = Some(Box::new(ListNode {
                val: n,
                next: result,
            }))
        }
        result
    }

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(reverse_list(list![]), list![]);
        assert_eq!(reverse_list(list![1]), list![1]);
        assert_eq!(reverse_list(list![1, 2]), list![2, 1]);
        assert_eq!(reverse_list(list![1, 2, 3]), list![3, 2, 1]);
    }
}

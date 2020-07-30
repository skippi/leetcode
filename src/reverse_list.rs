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
fn reverse_list(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! list {
        [] => {None};
        [$($x:expr), *] => {
            {
                let mut result: Option<Box<ListNode>> = None;
                let mut current = &mut result;
                $(
                    *current = Some($x);
                    current = current.unwrap().next;
                )*
                result
            }
        }
    }

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(reverse_list(list![]), list![]);
    }
}

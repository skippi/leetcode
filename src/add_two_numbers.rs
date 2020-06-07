#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
fn add_two_numbers(
    _l1: Option<Box<ListNode>>,
    _l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            add_two_numbers(make_list(&[2, 4, 3]), make_list(&[5, 6, 4])),
            make_list(&[7, 0, 8])
        );
    }

    fn make_list(slice: &[i32]) -> Option<Box<ListNode>> {
        slice.iter().rev().fold(None, |list, &elem| {
            Some(Box::new(ListNode {
                val: elem,
                next: list,
            }))
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode { val: 0, next: None });
    let mut current = &mut head;
    let mut remainder = 0;

    loop {
        let x = l1.iter().map(|node| node.val).next().unwrap_or(0);
        let y = l2.iter().map(|node| node.val).next().unwrap_or(0);
        let sum = x + y + remainder;
        current.val = sum % 10;
        remainder = sum / 10;

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);

        if l1.is_none() && l2.is_none() && remainder == 0 {
            break Some(head);
        }

        current.next = Some(Box::new(ListNode { val: 0, next: None }));
        current = current.next.as_mut().unwrap();
    }
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

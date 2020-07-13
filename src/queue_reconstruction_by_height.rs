/// Take the following input:
///
///   [[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]]
///
/// Sort it by highest height followed with lowest front count.
///
///   [[7,0], [7,1], [6,1], [5,0], [5,2], [4,4]]
///
/// Given this sorted list, it turns out that solving this problem involves treating the number of
/// people in front of a person as the insertion index. For example: [7,0] should be inserted at
/// index 0. Why?
///
/// Let's start by inserting the first item [7,0] at index 0. Since we start with an empty list,
/// our result becomes [[7,0]]. Now let's insert the next item [7,1] at index 1:
///
///   [[7,0], [7,1]]
///
/// Nothing too fancy... but what happens when we insert the next item [6,1] at index 1?
///
///   [[7,0], [6,1], [7,1]]
///
/// Our list order is correct, but how? The item [6,1] must be preceded by exactly 1 item that has a
/// height >= 6. The picture should be coming together here: if we insert at index 1, we'll be
/// inserting at a point that's preceded by 1 element. That's our exact requirement! To top it off,
/// the items are processed in order by highest-to-lowest height. This means that the previously
/// inserted items already satisfy the >= height condition.
///
/// Lastly, every insertion preserves the queue's order correctness. This means that we can
/// continuously insert each item to fold towards the finally ordered queue.
#[allow(dead_code)]
fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    people.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
    let mut queue = Vec::with_capacity(people.len());
    people
        .into_iter()
        .for_each(|person| queue.insert(person[1] as usize, person));
    queue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        assert_eq!(
            reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        )
    }
}

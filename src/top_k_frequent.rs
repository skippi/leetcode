use std::cmp::Reverse;
use std::collections::HashMap;

#[allow(dead_code)]
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();
    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mut counts = counts.into_iter().collect::<Vec<(i32, i32)>>();
    counts.sort_unstable_by_key(|(_, count)| Reverse(count.clone()));
    counts
        .into_iter()
        .map(|(n, _)| n)
        .take(k as usize)
        .collect()
}

/// Sort of doesn't work, long runtime
#[allow(dead_code)]
fn top_k_frequent_quicksearch(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }
    let mut counts = HashMap::new();
    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mut counts = counts.into_iter().collect::<Vec<(i32, i32)>>();
    (0..k as usize)
        .map(|i| {
            *quicksearch_by(&mut counts, i, |(n, count)| {
                (Reverse(count.clone()), n.clone())
            })
        })
        .map(|(n, _)| n)
        .collect()
}

fn quicksearch_by<T, K, F>(slice: &mut [T], index: usize, mut f: F) -> &mut T
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    if slice.is_empty() {
        panic!("cannot quicksearch empty slice");
    }
    let (left, middle, right) = partition_by(slice, &mut f);
    if index < left.len() {
        quicksearch_by(left, index, f)
    } else if index == left.len() {
        middle
    } else {
        quicksearch_by(right, index - left.len() - 1, f)
    }
}

fn partition_by<'a, T, K, F>(slice: &'a mut [T], f: &mut F) -> (&'a mut [T], &'a mut T, &'a mut [T])
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    if slice.is_empty() {
        panic!("cannot partition empty slice");
    }
    let mut index = 0;
    let (left, right) = slice.split_at_mut(slice.len() - 1);
    for i in 0..left.len() {
        if f(&left[i]).lt(&f(&right[0])) {
            left.swap(index, i);
            index += 1;
        }
    }
    slice.swap(index, slice.len() - 1);
    let (left, right) = slice.split_at_mut(index);
    let (middle, right) = right.split_at_mut(1);
    (left, &mut middle[0], right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![], 5), vec![]);
        assert_eq!(top_k_frequent(vec![1, 1, 2], 1), vec![1]);
    }

    #[test]
    fn test_top_k_frequent_quicksearch() {
        // assert_eq!(top_k_frequent_quicksearch(vec![], 5), vec![]);
        // assert_eq!(top_k_frequent_quicksearch(vec![1, 1, 2], 1), vec![1]);
        assert_eq!(top_k_frequent_quicksearch(vec![1, 2], 2), vec![1, 2]);
    }
}

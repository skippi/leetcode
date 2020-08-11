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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![], 5), vec![]);
        assert_eq!(top_k_frequent(vec![1, 1, 2], 1), vec![1]);
    }
}

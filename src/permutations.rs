#[allow(dead_code)]
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    match nums.len() {
        1 => vec![nums],
        _ => nums
            .iter()
            .flat_map(|&n| {
                let mut perm = permute(nums.iter().filter(|&&m| m != n).cloned().collect());
                for p in perm.iter_mut() {
                    p.insert(0, n);
                }
                perm
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
        assert_eq!(permute(vec![2]), vec![vec![2]]);
        assert_eq!(permute(vec![2, 1]), vec![vec![2, 1], vec![1, 2]]);
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

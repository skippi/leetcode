#[allow(dead_code)]
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    (0..nums.len() + 1)
        .flat_map(|k| combinations(&nums, k))
        .collect()
}

fn combinations(nums: &Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    let mut result = vec![];
    dfs(nums, k, 0, &mut vec![], &mut result);
    result
}

fn dfs(
    nums: &Vec<i32>,
    k: usize,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    for i in start..nums.len() {
        current.push(nums[i]);
        dfs(nums, k, i + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(subsets(vec![]), vec![vec![]]);
        assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            subsets(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
    }
}

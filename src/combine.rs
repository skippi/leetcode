#[allow(dead_code)]
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![];
    }
    let mut result = vec![];
    dfs(n, k, 0, &mut vec![], &mut result);
    result
}

fn dfs(n: i32, k: i32, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current.len() == k as usize {
        result.push(current.clone());
        return;
    }
    for i in start..n {
        current.push(1 + i);
        dfs(n, k, i + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        assert_eq!(combine(3, 0), Vec::<Vec<i32>>::new());
        assert_eq!(combine(3, 1), vec![vec![1], vec![2], vec![3]]);
        assert_eq!(combine(3, 2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
        assert_eq!(combine(0, 2), Vec::<Vec<i32>>::new());
    }
}

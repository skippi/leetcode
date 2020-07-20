#[allow(dead_code)]
fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn do_permute(nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, start: usize, end: usize) {
        if start == end {
            result.push(nums.clone());
            return;
        }
        for i in start..end {
            nums.swap(start, i);
            do_permute(nums, result, start + 1, end);
            nums.swap(start, i);
        }
    }
    let mut result = vec![];
    let length = nums.len();
    do_permute(&mut nums, &mut result, 0, length);
    result
}

#[allow(dead_code)]
fn permute_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
    match nums.len() {
        1 => vec![nums],
        _ => nums
            .iter()
            .flat_map(|&n| {
                let mut perm =
                    permute_recursive(nums.iter().filter(|&&m| m != n).cloned().collect());
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
    use std::collections::HashSet;
    use std::hash::Hash;

    #[test]
    fn test_permute() {
        run_tests(permute);
    }

    #[test]
    fn test_permute_recursive() {
        run_tests(permute_recursive);
    }

    fn run_tests<F: Fn(Vec<i32>) -> Vec<Vec<i32>>>(fun: F) {
        assert_eq!(fun(vec![1]), vec![vec![1]]);
        assert_eq!(fun(vec![2]), vec![vec![2]]);
        assert_eq!(fun(vec![2, 1]), vec![vec![2, 1], vec![1, 2]]);
        assert_eq!(
            to_set(fun(vec![1, 2, 3]).as_slice()),
            to_set(
                vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1]
                ]
                .as_slice()
            )
        );
    }

    fn to_set<C: Clone + Eq + Hash>(s: &[C]) -> HashSet<C> {
        s.iter().cloned().collect()
    }
}

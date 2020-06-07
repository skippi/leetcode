use std::collections::HashMap;

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_indices: HashMap<i32, usize> = HashMap::new();

    for (i, &x) in nums.iter().enumerate() {
        let result = num_indices.get(&(target - x)).filter(|&&j| i != j);

        if let Some(&j) = result {
            return vec![j as i32, i as i32];
        } else {
            num_indices.insert(x, i);
        }
    }

    vec![]
}

#[allow(dead_code)]
fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &x) in nums.iter().enumerate() {
        let result = nums
            .iter()
            .enumerate()
            .skip(1)
            .find(|&(j, &y)| x + y == target && i != j)
            .map(|(j, _)| vec![i as i32, j as i32]);

        if let Some(indices) = result {
            return indices;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_naive() {
        assert_eq!(two_sum_naive(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_naive(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum_naive(vec![2, 5, 5, 11], 10), vec![1, 2]);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
    }
}

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
    use std::collections::HashSet;

    macro_rules! set {
        [ $($x:expr),*] => {
                {
                    let mut temp_set = HashSet::new();
                    $(
                        temp_set.insert($x);
                    )*
                    temp_set
                }
        };
    }

    #[test]
    fn test_two_sum_naive() {
        two_sum_tests(two_sum_naive);
    }

    #[test]
    fn test_two_sum() {
        two_sum_tests(two_sum);
    }

    fn two_sum_tests<F>(fun: F)
    where
        F: Fn(Vec<i32>, i32) -> Vec<i32>,
    {
        assert_eq!(to_set(&fun(vec![2, 7, 11, 15], 9)), set![0, 1]);
        assert_eq!(to_set(&fun(vec![3, 2, 4], 6)), set![1, 2]);
        assert_eq!(to_set(&fun(vec![2, 5, 5, 11], 10)), set![1, 2]);
        assert_eq!(to_set(&fun(vec![0, 4, 3, 0], 0)), set![0, 3]);
    }

    fn to_set(s: &[i32]) -> HashSet<i32> {
        s.iter().cloned().collect()
    }
}

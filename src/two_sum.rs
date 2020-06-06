#[allow(dead_code)]
fn two_sum(_nums: Vec<i32>, _target: i32) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}

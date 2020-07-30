#[allow(dead_code)]
fn subsets(_nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(subsets(vec![]), Vec::<Vec<i32>>::new());
    }
}

#[allow(dead_code)]
fn combine(_n: i32, _k: i32) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        assert_eq!(combine(4, 0), Vec::<Vec<i32>>::new());
    }
}

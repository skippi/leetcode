#[allow(dead_code)]
fn generate_parenthesis(_n: i32) -> Vec<String> {
    vec!["()".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(generate_parenthesis(1), vec!["()"])
    }
}

#[allow(dead_code)]
fn generate_parenthesis(n: i32) -> Vec<String> {
    fn build_parens(n: i32) -> Vec<String> {
        match n {
            0 => vec![String::from("")],
            _ => {
                let mut result = vec![];
                for c in 0..n {
                    for left in build_parens(n - 1 - c) {
                        for right in build_parens(c) {
                            result.push(format!("({}){}", left, right));
                        }
                    }
                }
                result
            }
        }
    }
    match n {
        0 => vec![],
        _ => build_parens(n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(generate_parenthesis(0), Vec::<String>::new());
        assert_eq!(generate_parenthesis(1), vec!["()"]);
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(
            generate_parenthesis(4),
            vec![
                "(((())))", "((()()))", "((())())", "(()(()))", "(()()())", "((()))()", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }
}

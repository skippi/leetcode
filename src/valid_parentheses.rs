#[allow(dead_code)]
fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            ')' | ']' | '}' => {
                if let None = stack.pop().filter(|&e| e == c) {
                    return false;
                }
            }
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(!is_valid("[".to_string()));
    }
}

use std::cmp;
use std::collections::HashSet;

#[allow(dead_code)]
fn length_of_longest_substring(s: String) -> i32 {
    let mut result = 0usize;
    for i in 0..s.len() {
        let longest_window = (i..s.len())
            .map(|j| &s[i..j + 1])
            .take_while(|&w| is_unique(w))
            .last();

        if let Some(window) = longest_window {
            result = cmp::max(result, window.len())
        }
    }

    result as i32
}

fn is_unique(string: &str) -> bool {
    let mut chars = HashSet::new();
    for c in string.chars() {
        if chars.contains(&c) {
            return false;
        }

        chars.insert(c);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
    }

    #[test]
    fn test_is_unique() {
        assert_eq!(is_unique(" "), true);
    }
}

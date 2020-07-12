use std::cmp;
use std::collections::HashSet;

#[allow(dead_code)]
fn length_of_longest_substring(s: String) -> i32 {
    let mut char_next_indices = [0usize; 256]; // 256 ASCII chars
    let mut i = 0usize;
    let mut result = 0usize;

    for (j, letter) in s.char_indices() {
        i = cmp::max(i, char_next_indices[letter as usize]);
        char_next_indices[letter as usize] = j + 1;
        result = cmp::max(result, j - i + 1);
    }

    result as i32
}

#[allow(dead_code)]
fn length_of_longest_substring_naive(s: String) -> i32 {
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
    let mut seen = HashSet::new();
    string.chars().all(|c| seen.insert(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_naive() {
        apply_tests(length_of_longest_substring_naive);
    }

    #[test]
    fn test_is_unique() {
        assert_eq!(is_unique(" "), true);
        assert_eq!(is_unique("ada"), false);
    }

    #[test]
    fn test_length_of_longest_substring() {
        apply_tests(length_of_longest_substring);
    }

    fn apply_tests<F: Fn(String) -> i32>(fun: F) {
        assert_eq!(fun("abcabcbb".to_string()), 3);
        assert_eq!(fun("bbbbb".to_string()), 1);
        assert_eq!(fun("pwwkew".to_string()), 3);
        assert_eq!(fun(" ".to_string()), 1);
    }
}

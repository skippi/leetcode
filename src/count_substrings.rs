#[allow(dead_code)]
fn count_substrings(s: String) -> i32 {
    let mut result = 0;
    let n = s.len() as i32;
    let bytes = s.as_bytes();
    for center in 0i32..(2 * n - 1) {
        let mut left = center / 2;
        let mut right = left + center % 2;
        while 0 <= left && right < n && bytes[left as usize] == bytes[right as usize] {
            result += 1;
            left -= 1;
            right += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings() {
        assert_eq!(count_substrings("abc".to_string()), 3);
        assert_eq!(count_substrings("".to_string()), 0);
        assert_eq!(count_substrings("aa".to_string()), 3);
        assert_eq!(count_substrings("aaa".to_string()), 6);
    }
}

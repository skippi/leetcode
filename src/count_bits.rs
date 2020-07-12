use std::mem;

#[allow(dead_code)]
fn count_bits(num: i32) -> Vec<i32> {
    (0..num + 1).into_iter().map(|n| bit_count(n)).collect()
}

fn bit_count(num: i32) -> i32 {
    (0..mem::size_of::<i32>() * 8)
        .into_iter()
        .map(|index| 1 << index)
        .filter(|&mask| num & mask == mask)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}

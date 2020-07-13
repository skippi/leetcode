use std::mem;

/// Explanation:
/// - Let `n` be some number with `x >= 1` number of 1-bits. (EG: 5 [110] with 2 1-bits)
/// - The 1-bit count of `n` can be expressed as `(x - 1) + 1` (EG: (2 - 1) + 1)
/// - This means we only need to find the "previous" `n` that has one less
///   1-bit to find the bitcount of `n`.
/// - This "previous" number can be derived from `n & (n - 1)`.
///
///     - This bitwise expression outputs the current "suite" of a number. For example:
///
///       8  = 0b1000
///       9  = 0b1001
///       10 = 0b1010
///       11 = 0b1100
///
///       9 & (9 - 1)   = 0b1000
///       10 & (10 - 1) = 0b1000
///       11 & (11 - 1) = 0b1000
///       12 & (12 - 1) = 0b1100
///
///       Notice how the right-most `1` bubbles up to the left of the binary representation. This
///       left part always has consecutive ones and stays consistent within the "suite" (even
///       though the rightmost `1` is still bubbling up, this suite stays consistent).
///
/// - It's also the case that the "suite" is always going to be less than the current `n`.
///   Because of this, we can derive `n`'s bitcount from previously computed `n`s.
#[allow(dead_code)]
fn count_bits(num: i32) -> Vec<i32> {
    let num = num as usize;
    let mut bit_counts = vec![0; num + 1];
    for n in 1..num + 1 {
        let suite = n & (n - 1);
        bit_counts[n] = bit_counts[suite] + 1;
    }
    bit_counts
}

#[allow(dead_code)]
fn count_bits_naive(num: i32) -> Vec<i32> {
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
        run_tests(count_bits);
    }

    #[test]
    fn test_count_bits_naive() {
        run_tests(count_bits_naive);
    }

    fn run_tests<F: Fn(i32) -> Vec<i32>>(fun: F) {
        assert_eq!(fun(2), vec![0, 1, 1]);
        assert_eq!(fun(5), vec![0, 1, 1, 2, 1, 2]);
    }
}

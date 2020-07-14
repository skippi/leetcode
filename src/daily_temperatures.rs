/// This algorithm is a bit tricky to explain. For this explanation, let's use this example input:
///
/// [68, 69, 71, 70, 69, 72, 69, 68, 70]
///
/// Since 68 < 69 < 71, it's fairly clear that 68 and 69 need to wait for only 1 day to reach a
/// hotter temperature. This isn't too bad.
///
/// However, 71 must wait 3 days to reach a hotter temperature. Reaching this conclusion can be
/// done through a naive O(n^2) method, but can it be done with lower time complexity?
///
/// Spoiler: it can be, but we should explore the data format to understand why. Let's start by
/// listing out the next hottest days for each temperature in order:
///
/// 68 -> 69
/// 69 -> 71
/// 71 -> 72
/// 70 -> 72
/// 69 -> 72
/// 72 -> None
/// 69 -> 70
/// 68 -> 70
/// 70 -> None
///
/// It appears that some of the temperatures share the same next hottest temperature (EG: 71, 70,
/// and 69 share 72). If we can find a way to store these same temperatures (like 72 and 70) to
/// assign them, we'd be able to solve our problem much more efficiently. Let's start by addressing
/// some patterns to gauge our problem better.
///
/// One pattern to notice is that the next hottest temperature always occurs afterwards for each
/// day. This hints at a small suggestion: it might be better to iterate from the end of the input.
///
/// Something else to notice is that the next hottest temperature never occurs on the day of;
/// rather that it always happens afterwards. This is most prevalent with 72 and 70.
///
/// Given this information, there is a small link to our data format. Let's look at [69, 68, 70].
/// Say we iterated from end to beginning. Let's make it a goal to figure out each day's next
/// hottest temperature. To do this for these three days, we essentially discarded [68, 69] to
/// reach 70 for all of these. Modeling this sort of discarding can be done with a stack structure.
///
/// 70 -> []        | None
///    -> [70]
/// 68 -> [70]      | 70
///    -> [70, 68]
/// 69 -> [70, 68]
///    -> [70]      | 70
///    -> [70, 69]
/// 72 -> [70, 69]
///    -> [70]
///    -> []        | None
///
/// This stack-based algorithm leads us to our non-naive solution:
#[allow(dead_code)]
fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; t.len()];
    let mut days_stack = vec![];
    for i in (0..t.len()).rev() {
        while !days_stack.is_empty() && t[*days_stack.last().unwrap()] <= t[i] {
            days_stack.pop();
        }
        result[i] = match days_stack.last() {
            Some(j) => (j - i) as i32,
            None => 0,
        };
        days_stack.push(i);
    }
    result
}

#[allow(dead_code)]
fn daily_temperatures_naive(t: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..t.len() {
        let search = (i..t.len()).find(|&j| t[j] > t[i]);
        let days_away = match search {
            Some(j) => j - i,
            None => 0,
        };
        result.push(days_away as i32);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        run_tests(daily_temperatures);
    }

    #[test]
    fn test_daily_temperatures_naive() {
        run_tests(daily_temperatures_naive);
    }

    fn run_tests<F: Fn(Vec<i32>) -> Vec<i32>>(fun: F) {
        assert_eq!(
            fun(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}

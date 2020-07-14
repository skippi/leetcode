#[allow(dead_code)]
fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut days_stack = vec![];
    for (i, &temperature) in t.iter().enumerate().rev() {
        while !days_stack.is_empty() && t[*days_stack.last().unwrap()] <= temperature {
            days_stack.pop();
        }
        let days_away = match days_stack.last() {
            Some(j) => j - i,
            None => 0,
        };
        result.push(days_away as i32);
        days_stack.push(i);
    }
    result.reverse();
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

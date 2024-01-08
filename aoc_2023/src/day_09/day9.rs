fn get_next_value(history: Vec<i32>, rev: bool) -> i32 {
    if history.iter().all(|v| *v == 0) {
        return 0;
    }

    let next_values = history.windows(2).map(|v| v[1] - v[0]).collect();
    if rev {
        return history.first().unwrap() - get_next_value(next_values, rev);
    }

    get_next_value(next_values, rev) + history.last().unwrap()
}

pub fn solve_q9_p1(s: &str) -> i32 {
    s.split('\n')
        .map(|l| {
            get_next_value(
                l.split(' ').map(|v| v.parse::<i32>().unwrap()).collect(),
                false,
            )
        })
        .sum()
}

pub fn solve_q9_p2(s: &str) -> i32 {
    s.split('\n')
        .map(|l| {
            get_next_value(
                l.split(' ').map(|v| v.parse::<i32>().unwrap()).collect(),
                true,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An input and expected return
    struct TestCase<T, U>(T, U);

    #[test]
    fn test_get_next_value() {
        let test_cases = vec![
            TestCase(String::from("0 3 6 9 12 15"), 18),
            TestCase(String::from("1 3 6 10 15 21"), 28),
            TestCase(String::from("10 13 16 21 30 45"), 68),
        ];

        for case in test_cases {
            let result = get_next_value(
                case.0
                    .split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect(),
                false,
            );
            assert_eq!(
                result, case.1,
                "get_next_value(\"{}\", false): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_get_next_value_rev() {
        let test_cases = vec![
            TestCase(String::from("0 3 6 9 12 15"), -3),
            TestCase(String::from("1 3 6 10 15 21"), 0),
            TestCase(String::from("10 13 16 21 30 45"), 5),
        ];

        for case in test_cases {
            let result = get_next_value(
                case.0
                    .split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect(),
                true,
            );
            assert_eq!(
                result, case.1,
                "get_next_value(\"{}\", true): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q9p1() {
        let d9p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q9_p1(d9p1_test), 114);
    }

    #[test]
    fn test_solve_q9p2() {
        let d9p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q9_p2(d9p2_test), 2);
    }

    #[test]
    fn test_answers() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_q9_p1(input), 1637452029);
        assert_eq!(solve_q9_p2(input), 908);
    }
}

pub fn solve_q1_p1(s: &str) -> u32 {
    // expected input is the entire input file as an &str
    s.split('\n').map(|l| calibration_value(l)).sum()
}

pub fn solve_q1_p2(s: &str) -> u32 {
    // expected input is the entire input file as an &str
    s.split('\n')
        .map(|l| calibration_value(&replace_words(l)))
        .sum()
}

fn replace_words(s: &str) -> String {
    // Do this funny replace because cases like "oneight" is supposed to be 18 and not 11
    let words: Vec<(&str, &str)> = vec![
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "thr3ee"),
        ("four", "4"),
        ("five", "fiv5e"),
        ("six", "6"),
        ("seven", "seve7n"),
        ("eight", "eigh8t"),
        ("nine", "nin9e"),
    ];

    let mut result: String = String::from(s);

    for word in words {
        result = result.replace(word.0, word.1)
    }

    result
}

fn calibration_value(s: &str) -> u32 {
    // remove alphabetic charcters
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    if digits == "" {
        return 0;
    }

    // first and last digit
    let tens = digits.chars().next().unwrap().to_digit(10).unwrap() * 10;
    let ones = digits.chars().last().unwrap().to_digit(10).unwrap();

    tens + ones
}

#[cfg(test)]
mod tests {
    use crate::day_01::day1::{calibration_value, replace_words, solve_q1_p1, solve_q1_p2};

    /// An input and expected return
    struct TestCase<T, U>(T, U);

    #[test]
    fn test_calibration_value() {
        let test_cases = vec![
            TestCase(String::from("1abc2"), 12),
            TestCase(String::from("pqr3stu8vwx"), 38),
            TestCase(String::from("a1b2c3d4e5f"), 15),
            TestCase(String::from("treb7uchet"), 77),
            TestCase(String::from(""), 0),
            TestCase(String::from("1"), 11),
            TestCase(String::from("a"), 0),
        ];

        for case in test_cases {
            let result = calibration_value(&case.0);
            assert_eq!(
                result, case.1,
                "calibration_value(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_p1() {
        let d1p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q1_p1(d1p1_test), 142);
    }

    #[test]
    fn test_replace_words() {
        let test_cases = vec![
            TestCase(String::from("two1nine"), "tw2o1nin9e"),
            TestCase(String::from("eightwothree"), "eigh8tw2othr3ee"),
            TestCase(String::from("abcone2threexyz"), "abcon1e2thr3eexyz"),
            TestCase(String::from("xtwone3four"), "xtw2on1e34"),
            TestCase(String::from("4nineeightseven2"), "4nin9eeigh8tseve7n2"),
            TestCase(String::from("zoneight234"), "zon1eigh8t234"),
            TestCase(String::from("7pqrstsixteen"), "7pqrst6teen"),
            TestCase(String::from("1"), "1"),
            TestCase(String::from(""), ""),
            TestCase(String::from("a"), "a"),
            TestCase(String::from("oneight"), "on1eigh8t"),
        ];

        for case in test_cases {
            let result = replace_words(&case.0);
            assert_eq!(
                result, case.1,
                "replace_words(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_p2_lines() {
        let test_cases = vec![
            TestCase(String::from("two1nine"), 29),
            TestCase(String::from("eightwothree"), 83),
            TestCase(String::from("abcone2threexyz"), 13),
            TestCase(String::from("xtwone3four"), 24),
            TestCase(String::from("4nineeightseven2"), 42),
            TestCase(String::from("zoneight234"), 14),
            TestCase(String::from("7pqrstsixteen"), 76),
            TestCase(String::from("1"), 11),
            TestCase(String::from(""), 0),
            TestCase(String::from("a"), 0),
            TestCase(String::from("oneight"), 18), // this type is the tricky one!
        ];

        for case in test_cases {
            let result = solve_q1_p2(&case.0);
            assert_eq!(
                result, case.1,
                "solve_q1_p2(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_p2() {
        let d1p2_test = include_str!("./input2_test.txt");
        assert_eq!(solve_q1_p2(d1p2_test), 281);
    }
}

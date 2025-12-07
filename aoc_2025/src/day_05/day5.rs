pub fn part1(input: &str) -> u64 {
    let input: Vec<&str> = input.splitn(2, "\n\n").collect();
    let (ranges, values) = (input.first().unwrap(), input.last().unwrap());
    let ranges: Vec<&str> = ranges.lines().collect();
    let values: Vec<&str> = values.lines().collect();
    let mut total = 0;
    for value in values {
        if value_in_range(value.parse().unwrap(), &ranges) > 0 {
            total += 1;
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    1
}

// given a value and a lite of ranges, returns how many ranges contain the value
fn value_in_range(value: u64, ranges: &[&str]) -> u64 {
    let mut total = 0;
    for range in ranges {
        let (lower, upper) = range.split_once('-').unwrap();
        if value >= lower.parse().unwrap() && value <= upper.parse().unwrap() {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day_05::day5::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(3, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}

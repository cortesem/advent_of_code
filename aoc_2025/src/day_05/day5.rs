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
    // Turn the input string into a vec of the ranges as tuples.
    let mut input: Vec<(u64, u64)> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    // Sort the ranges by the start of the range
    input.sort_by_key(|l| l.0);

    let mut merged_ranges = Vec::<(u64, u64)>::new();

    // Merge and overlapping ranges
    for (start, end) in input {
        if let Some((_, prev_end)) = merged_ranges.last_mut()
            && start <= *prev_end + 1
        {
            *prev_end = (*prev_end).max(end);
            continue;
        }
        // If we didn't merge a range, then this is a new range
        merged_ranges.push((start, end));
    }

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
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
        assert_eq!(14, part2(input));
    }
}

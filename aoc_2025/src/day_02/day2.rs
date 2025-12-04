pub fn part1(input: &str) -> u64 {
    // let id_sum = 0;
    // let ranges: Vec<&str> =

    input.trim().split(',').map(count_invalid_ids).sum()
}

pub fn part2(input: &str) -> u64 {
    input.trim().split(',').map(count_invalid_ids).sum()
}

fn count_invalid_ids(id_range: &str) -> u64 {
    let mut sum = 0;

    let (low, high) = id_range.split_once('-').unwrap();
    let split_index = if low.len() == 1 { 1 } else { low.len() / 2 };
    // split this number in half to make pattern creation easier
    let mut id = low[..split_index]
        .parse::<u64>()
        .expect("expected a number!");

    // this gets us the number of digits in id, we'll need it in cases when we need to increase by
    // powers of 10
    let mut num_digits = id.ilog10() + 1;

    // Patterns can only exist with even number of digits, except for single digits.
    if low.len() == 1 {
        // this is a special case for single digit patterns
        id = 1;
    } else if low.len() % 2 != 0 {
        // we need to bump this bad boy up to the next nearest power of 10. ie. 123 should be
        // bumped to 1000 since thats the lowest next possible pattern.
        id = 10u64.pow(num_digits);
        num_digits += 1;
    }

    let high = high.parse::<u64>().expect("expected a number!");
    let low = low.parse::<u64>().expect("expected a number!");

    // ensure our first possible pattern is above the min. ex. 1299 will start at 1212 -> too low.
    let mut pattern = id * 10u64.pow(num_digits) + id;
    while pattern < low {
        id += 1;
        num_digits = id.ilog10() + 1;
        pattern = id * 10u64.pow(num_digits) + id;
    }

    // now we can start checking for and counting patterns, until our pattern is larger than the
    // high end of the range.
    while pattern <= high {
        sum += pattern;
        id += 1;
        num_digits = id.ilog10() + 1;
        pattern = id * 10u64.pow(num_digits) + id;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_02::day2::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1227775554, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(4174379265, part2(input));
    }
}

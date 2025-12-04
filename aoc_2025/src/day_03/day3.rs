pub fn part1(input: &str) -> u64 {}

pub fn part2(_input: &str) -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::day_02::day2::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(357, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|bank| {
            let (f, s) = top_two(bank);
            f * 10 + s
        })
        .sum()
}

pub fn part2(_input: &str) -> u64 {
    1
}

fn top_two(numbers: &str) -> (u64, u64) {
    let numbers: Vec<u64> = numbers
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut first = numbers.first().unwrap();
    let mut index = 0;
    for i in 1..numbers.len() - 1 {
        let num = numbers.get(i).unwrap();
        if num > first {
            first = num;
            index = i;
        }
    }

    let mut second = numbers.get(index + 1).unwrap();
    for i in (index + 2)..numbers.len() {
        let num = numbers.get(i).unwrap();
        if num > second {
            second = num;
        }
    }

    (*first, *second)
}

#[cfg(test)]
mod tests {
    use crate::day_03::day3::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(357, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(3121910778619, part2(input));
    }
}

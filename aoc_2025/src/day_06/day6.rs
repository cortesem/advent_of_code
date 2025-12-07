pub fn part1(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split('\n').collect();
    // last line is the math signs
    let (signs, values) = input.split_last().unwrap();
    let signs: Vec<&str> = signs.split_whitespace().collect();
    // start the totals array with the first line of values
    let (totals, values) = values.split_first().unwrap();
    let mut totals: Vec<u64> = totals
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    for value in values {
        let _: Vec<u64> = value
            .split_whitespace()
            .enumerate()
            .map(|(i, n)| {
                let n: u64 = n.parse().unwrap();
                let s = signs.get(i).unwrap();
                if *s == "+" {
                    totals[i] += n;
                } else {
                    totals[i] *= n;
                }
                n
            })
            .collect();
    }

    totals.iter().sum()
}

pub fn part2(input: &str) -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::day_06::day6::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(4277556, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}

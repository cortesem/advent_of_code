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

pub fn part2(input: &str) -> u64 {
    input.trim().split('\n').map(|bank| topk(bank, 12)).sum()
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

fn topk(numbers: &str, k: usize) -> u64 {
    let mut numbers: Vec<(usize, u64)> = numbers
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as u64))
        .collect();

    numbers.sort_by(|a, b| b.1.cmp(&a.1));
    // numbers.sort_by_key(|(_, num)| *num);
    println!("{:?}", numbers);

    // now take the topk and sort by index
    let mut numbers: Vec<(usize, u64)> = numbers.into_iter().take(k).collect();
    numbers.sort_by_key(|(index, _)| *index);
    println!("{:?}", numbers);

    let mut combined = numbers.first().unwrap().1;
    for i in 1..numbers.len() {
        combined *= 10;
        combined += numbers.get(i).unwrap().1;
    }
    println!("{combined}");

    combined
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

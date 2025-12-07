pub fn part1(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split('\n').collect();
    // last line is the math signs
    let (operators, values) = input.split_last().unwrap();
    let operators: Vec<&str> = operators.split_whitespace().collect();
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
                let s = operators.get(i).unwrap();
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
    let input: Vec<&str> = input.trim().split('\n').collect();
    // last line is the math signs
    let (operators, input) = input.split_last().unwrap();
    let operators: Vec<&str> = operators.split_whitespace().collect();
    let input: Vec<Vec<char>> = input.iter().map(|v| v.chars().collect()).collect();

    // go through values and build all of the numbers. Put each set of numbers in their own array,
    // such that and operator works on a single array.
    let line = input.first().unwrap();
    let mut values = Vec::new();
    let mut current_values = Vec::new();
    for (i, c) in line.iter().enumerate() {
        let mut num = c.to_string();
        for j in 1..input.len() {
            let c_below = input.get(j).unwrap().get(i).unwrap();
            if *c_below != ' ' {
                num = format!("{num}{c_below}");
            }
        }
        if num != " " {
            let num: u64 = num.trim().parse().unwrap();
            current_values.push(num);
        }
        if num == " " || i == line.len() - 1 {
            // we have hit a colum of whitespace ie. the seperator.
            values.push(current_values);
            current_values = Vec::new();
        }
    }

    let mut total = 0;
    for (i, nums) in values.iter().enumerate() {
        let mut current = *nums.first().unwrap();
        let op = operators.get(i).unwrap();
        for j in 1..nums.len() {
            let num = nums.get(j).unwrap();
            if *op == "+" {
                current += num;
            } else {
                current *= num;
            }
        }
        total += current;
    }

    total
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
        assert_eq!(3263827, part2(input));
    }
}

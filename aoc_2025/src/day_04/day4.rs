pub fn part1(input: &str) -> u64 {
    let input: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    count_adjacent_rolls(&input)
}

pub fn part2(input: &str) -> u64 {
    let height = input.lines().count();
    let mut input: Vec<char> = input.trim().lines().flat_map(|line| line.chars()).collect();
    let mut rolls_rm = 1;
    let mut total = 0;
    while rolls_rm != 0 {
        rolls_rm = remove_rolls(&mut input, height);
        total += rolls_rm;
    }

    total
}

pub fn count_adjacent_rolls(input: &[Vec<char>]) -> u64 {
    let mut total = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item != '@' {
                continue;
            }

            let mut adj_rolls = 0;
            // check above
            if i > 0 {
                // left
                if j > 0 && input[i - 1][j - 1] == '@' {
                    adj_rolls += 1;
                }
                // right
                if j < row.len() - 1 && input[i - 1][j + 1] == '@' {
                    adj_rolls += 1;
                }
                // mid
                if input[i - 1][j] == '@' {
                    adj_rolls += 1;
                }
            }

            // check below
            if i < input.len() - 1 {
                // left
                if j > 0 && input[i + 1][j - 1] == '@' {
                    adj_rolls += 1;
                }
                // right
                if j < row.len() - 1 && input[i + 1][j + 1] == '@' {
                    adj_rolls += 1;
                }
                // mid
                if input[i + 1][j] == '@' {
                    adj_rolls += 1;
                }
            }

            // left
            if j > 0 && input[i][j - 1] == '@' {
                adj_rolls += 1;
            }
            // right
            if j < row.len() - 1 && input[i][j + 1] == '@' {
                adj_rolls += 1;
            }

            if adj_rolls < 4 {
                total += 1;
            }
        }
    }

    total
}

pub fn remove_rolls(input: &mut [char], height: usize) -> u64 {
    let width = input.len() / height;
    let mut total = 0;

    for i in 0..input.len() {
        if input[i] == '.' {
            continue;
        }

        let x = i % width;
        let y = i / width;
        let mut adj_rolls = 0;

        // up
        if y > 0 {
            // left
            if x > 0 && input[i - width - 1] == '@' {
                adj_rolls += 1
            }
            // right
            if x < width - 1 && input[i - width + 1] == '@' {
                adj_rolls += 1
            }

            if input[i - width] == '@' {
                adj_rolls += 1
            }
        }
        // down
        if y < height - 1 {
            // left
            if x > 0 && input[i + width - 1] == '@' {
                adj_rolls += 1
            }
            // right
            if x < width - 1 && input[i + width + 1] == '@' {
                adj_rolls += 1
            }

            if input[i + width] == '@' {
                adj_rolls += 1
            }
        }
        // left
        if x > 0 && input[i - 1] == '@' {
            adj_rolls += 1
        }
        // right
        if x < width - 1 && input[i + 1] == '@' {
            adj_rolls += 1
        }

        // check if we remove this roll
        if adj_rolls < 4 {
            total += 1;
            input[i] = '.';
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day_04::day4::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(13, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(43, part2(input));
    }
}

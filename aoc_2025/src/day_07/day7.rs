pub fn part1(input: &str) -> u64 {
    // Need to flatten the 2dness of this puzzle
    let height = input.lines().count();
    let mut input: Vec<char> = input.trim().lines().flat_map(|line| line.chars()).collect();
    let width = input.len() / height;

    // find the starting point
    let mut start = 0;
    for i in 0..width {
        if *input.get(i).unwrap() == 'S' {
            start = i;
        }
    }

    // depth first style search
    let mut beams = vec![start];
    let mut splits = 0;

    while let Some(mut current_beam) = beams.pop() {
        // search down until we hit a split, or the bottom of the manifold
        while current_beam / width < height - 1 {
            // we already hit this splitter.
            if input[current_beam] == 'x' {
                break;
            }

            if input[current_beam] == '^' {
                splits += 1;
                // Mark this splitter in case we hit it again later.
                input[current_beam] = 'x';
                // Mark the beams so we don't double count them when hitting adjacent splitters.
                if input[current_beam - 1] != '|' {
                    beams.push(current_beam - 1);
                    input[current_beam - 1] = '|';
                }
                if input[current_beam + 1] != '|' {
                    beams.push(current_beam + 1);
                    input[current_beam + 1] = '|';
                }
                break;
            } else {
                current_beam += width;
            }
        }
    }

    splits
}

pub fn part2(input: &str) -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::day_07::day7::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(21, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}

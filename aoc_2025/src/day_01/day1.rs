pub fn part1(input: &str) -> u32 {
    let mut dial = 50;
    let mut code = 0;

    let rotations: Vec<&str> = input.trim().split('\n').collect();
    for rot in rotations {
        dial = rotate_dial(dial, rot);
        if dial == 0 {
            code += 1;
        }
    }

    code
}

// Takes the dial and the next rotation. Returns the dial rotated by rotation.
fn rotate_dial(mut dial: u32, rotation: &str) -> u32 {
    // Assume every dial has 100 values
    let (direction, value) = rotation.split_at(1);
    let value = value.parse::<u32>().expect("expected a number");

    if direction == "L" {
        dial = dial + 100 - (value % 100);
    } else {
        dial += value;
    }

    dial % 100
}

#[cfg(test)]
mod tests {
    use crate::day_01::day1::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(3, part1(input));
    }
}

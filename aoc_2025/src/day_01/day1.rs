pub fn part1(input: &str) -> i32 {
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

pub fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut code = 0;

    let rotations: Vec<&str> = input.trim().split('\n').collect();
    for rot in rotations {
        let count;
        (dial, count) = count_rotations(dial, rot);
        code += count;
    }

    code
}

// Takes the dial and the next rotation. Returns the dial rotated by rotation.
fn rotate_dial(dial: i32, rotation: &str) -> i32 {
    // Assume every dial has 100 values
    let (direction, value) = rotation.split_at(1);
    let mut value = value.parse::<i32>().expect("expected a number");

    if direction == "L" {
        value = -value;
    }

    (dial + value) % 100
}

// Takes the dial and the next rotation. Returns the dial rotated by rotation and the number of
// times the dial touched 0.
fn count_rotations(mut dial: i32, rotation: &str) -> (i32, i32) {
    // Assume every dial has 100 values
    let (direction, value) = rotation.split_at(1);
    let mut value = value.parse::<i32>().expect("expected a number");
    let mut count = 0;

    if value >= 100 {
        count = (value / 100).abs();
        value %= 100;
    }

    if direction == "L" {
        if dial == 0 {
            dial -= value;
            if dial < 0 {
                dial += 100;
            }
        } else {
            dial -= value;
            if dial <= 0 {
                count += 1;
                if dial < 0 {
                    dial += 100;
                }
            }
        }
    } else {
        dial += value;
        if dial >= 100 {
            count += 1;
            dial -= 100;
        }
    }

    (dial, count)
}

#[cfg(test)]
mod tests {
    use crate::day_01::day1::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(3, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(6, part2(input));
    }
}

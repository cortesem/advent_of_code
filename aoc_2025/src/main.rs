use crate::day_01::*;

mod day_01;

fn main() {
    println!(
        "Day1 part1: {}",
        day1::part1(include_str!("day_01/input/input.txt"))
    );

    println!(
        "Day1 part2: {}",
        day1::part2(include_str!("day_01/input/input.txt"))
    );
}

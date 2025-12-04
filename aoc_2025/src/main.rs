use crate::day_01::*;
use crate::day_02::*;

mod day_01;
mod day_02;

fn main() {
    println!(
        "Day1 part1: {}",
        day1::part1(include_str!("day_01/input/input.txt"))
    );

    println!(
        "Day1 part2: {}",
        day1::part2(include_str!("day_01/input/input.txt"))
    );

    println!(
        "Day2 part1: {}",
        day2::part1(include_str!("day_02/input/input.txt"))
    );

    println!(
        "Day2 part2: {}",
        day2::part2(include_str!("day_02/input/input.txt"))
    );
}

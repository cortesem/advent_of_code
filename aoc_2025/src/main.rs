use crate::day_01::*;
use crate::day_02::*;
use crate::day_03::*;
use crate::day_04::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

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

    println!(
        "Day3 part1: {}",
        day3::part1(include_str!("day_03/input/input.txt"))
    );

    println!(
        "Day3 part2: {}",
        day3::part2(include_str!("day_03/input/input.txt"))
    );

    println!(
        "Day4 part1: {}",
        day4::part1(include_str!("day_04/input/input.txt"))
    );

    println!(
        "Day4 part2: {}",
        day4::part2(include_str!("day_04/input/input.txt"))
    );
}

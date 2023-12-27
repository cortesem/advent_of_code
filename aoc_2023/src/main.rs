use crate::{
    day_01::day1::{solve_q1_p1, solve_q1_p2},
    day_02::day2::{solve_q2_p1, solve_q2_p2},
    day_03::day3::{solve_q3_p1, solve_q3_p2},
};

mod day_01;
mod day_02;
mod day_03;

fn main() {
    println!(
        "Day1 Part1: {}",
        solve_q1_p1(include_str!("./day_01/input.txt"))
    );
    println!(
        "Day1 Part2: {}",
        solve_q1_p2(include_str!("./day_01/input.txt"))
    );
    println!(
        "Day2 Part1: {}",
        solve_q2_p1(include_str!("./day_02/input.txt"))
    );
    println!(
        "Day2 Part2: {}",
        solve_q2_p2(include_str!("./day_02/input.txt"))
    );
    println!(
        "Day3 Part1: {}",
        solve_q3_p1(include_str!("./day_03/input.txt"))
    );
    println!(
        "Day3 Part2: {}",
        solve_q3_p2(include_str!("./day_03/input.txt"))
    );
}

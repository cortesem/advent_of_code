use crate::{
    day_01::day1::{solve_q1_p1, solve_q1_p2},
    day_02::day2::{solve_q2_p1, solve_q2_p2},
    day_03::day3::{solve_q3_p1, solve_q3_p2},
    day_04::day4::{solve_q4_p1, solve_q4_p2},
    day_05::day5::{solve_q5_p1, solve_q5_p2},
    day_06::day6::{solve_q6_p1, solve_q6_p2},
};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

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
    println!(
        "Day4 Part1: {}",
        solve_q4_p1(include_str!("./day_04/input.txt"))
    );
    println!(
        "Day4 Part2: {}",
        solve_q4_p2(include_str!("./day_04/input.txt"))
    );
    println!(
        "Day5 Part1: {}",
        solve_q5_p1(include_str!("./day_05/input.txt"))
    );
    println!(
        "Day5 Part2: {}",
        solve_q5_p2(include_str!("./day_05/input.txt"))
    );
    println!(
        "Day6 Part1: {}",
        solve_q6_p1(include_str!("./day_06/input.txt"))
    );
    println!(
        "Day6 Part2: {}",
        solve_q6_p2(include_str!("./day_06/input.txt"))
    );
}

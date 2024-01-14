use std::time::Instant;

use crate::{
    day_01::day1::{solve_q1_p1, solve_q1_p2},
    day_02::day2::{solve_q2_p1, solve_q2_p2},
    day_03::day3::{solve_q3_p1, solve_q3_p2},
    day_04::day4::{solve_q4_p1, solve_q4_p2},
    day_05::day5::{solve_q5_p1, solve_q5_p2},
    day_06::day6::{solve_q6_p1, solve_q6_p2},
    day_07::day7::{solve_q7_p1, solve_q7_p2},
    day_08::day8::{solve_q8_p1, solve_q8_p2},
    day_09::day9::{solve_q9_p1, solve_q9_p2},
    day_10::day10::{solve_q10_p1, solve_q10_p2},
    day_11::day11::{solve_q11_p1, solve_q11_p2},
};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

fn main() {
    let total = Instant::now();
    let mut start = Instant::now();
    println!(
        "Day1 Part1: {}\t\t{:.2?}",
        solve_q1_p1(include_str!("./day_01/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day1 Part2: {}\t\t{:.2?}",
        solve_q1_p2(include_str!("./day_01/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day2 Part1: {}\t\t{:.2?}",
        solve_q2_p1(include_str!("./day_02/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day2 Part2: {}\t\t{:.2?}",
        solve_q2_p2(include_str!("./day_02/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day3 Part1: {}\t\t{:.2?}",
        solve_q3_p1(include_str!("./day_03/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day3 Part2: {}\t\t{:.2?}",
        solve_q3_p2(include_str!("./day_03/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day4 Part1: {}\t\t{:.2?}",
        solve_q4_p1(include_str!("./day_04/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day4 Part2: {}\t\t{:.2?}",
        solve_q4_p2(include_str!("./day_04/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day5 Part1: {}\t\t{:.2?}",
        solve_q5_p1(include_str!("./day_05/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day5 Part2: {}\t\t{:.2?}",
        solve_q5_p2(include_str!("./day_05/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day6 Part1: {}\t\t{:.2?}",
        solve_q6_p1(include_str!("./day_06/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day6 Part2: {}\t\t{:.2?}",
        solve_q6_p2(include_str!("./day_06/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day7 Part1: {}\t\t{:.2?}",
        solve_q7_p1(include_str!("./day_07/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day7 Part2: {}\t\t{:.2?}",
        solve_q7_p2(include_str!("./day_07/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day8 Part1: {}\t\t{:.2?}",
        solve_q8_p1(include_str!("./day_08/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day8 Part2: {}\t{:.2?}",
        solve_q8_p2(include_str!("./day_08/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day9 Part1: {}\t\t{:.2?}",
        solve_q9_p1(include_str!("./day_09/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day9 Part2: {}\t\t\t{:.2?}",
        solve_q9_p2(include_str!("./day_09/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day10 Part1: {}\t\t{:.2?}",
        solve_q10_p1(include_str!("./day_10/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day10 Part2: {}\t\t{:.2?}",
        solve_q10_p2(include_str!("./day_10/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day11 Part1: {}\t\t{:.2?}",
        solve_q11_p1(include_str!("./day_11/input.txt")),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Day11 Part2: {}\t{:.2?}",
        solve_q11_p2(include_str!("./day_11/input.txt")),
        start.elapsed()
    );

    println!("Total time:\t\t\t{:.2?}", total.elapsed());
}

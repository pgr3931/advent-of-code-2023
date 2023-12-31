mod utils;

mod day1;
use day1::solve as day1_solve;

mod day3;
use day3::solve as day3_solve;

mod day5;
use day5::solve as day5_solve;

mod day7;
use day7::solve as day7_solve;

mod day9;
use day9::solve as day9_solve;

mod day11;
use day11::solve as day11_solve;

mod day13;
use day13::solve as day13_solve;

mod day15;
use day15::solve as day15_solve;

mod day17;
use day17::solve as day17_solve;

mod day19;
use day19::solve as day19_solve;

mod day21;
use day21::solve as day21_solve;

mod day23;
use day23::solve as day23_solve;

mod naughty_even_days;
use naughty_even_days::day10::solve as day10_solve;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let day = args[1].parse::<i32>().unwrap();
    let run_as = args[2].chars().nth(0).unwrap();

    match match day {
        1 => day1_solve(run_as),
        3 => day3_solve(run_as),
        5 => day5_solve(run_as),
        7 => day7_solve(run_as),
        9 => day9_solve(run_as),
        10 => day10_solve(run_as),
        11 => day11_solve(run_as),
        13 => day13_solve(run_as),
        15 => day15_solve(run_as),
        17 => day17_solve(run_as),
        19 => day19_solve(run_as),
        21 => day21_solve(run_as),
        23 => day23_solve(run_as),
        _ => Ok(()),
    } {
        Err(err) => println!("{}", err),
        _ => {}
    }
}

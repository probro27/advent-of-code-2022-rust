mod file_utils;
mod solutions;

use std::env;

pub struct Solution {
    first: String,
    second: String
}

fn solve(day: i32, input: String) -> Solution {
    match day {
        1 => solutions::day01::solve(input),
        2 => solutions::day02::solve(input),
        3 => solutions::day03::solve(input),
        4 => solutions::day04::solve(input),
        5 => solutions::day05::solve(input),
        _ => panic!("This day doesn't exist")
    }
}

fn get_day_arg() -> i32 {
    let args: Vec<String> = env::args().collect();
    args[1].parse::<i32>().unwrap()
}

fn main() {
    let day = get_day_arg();
    let input = file_utils::get_input(day);
    let solution = solve(day, input);

    println!("First: {}", solution.first);
    println!("Second: {}", solution.second);
}

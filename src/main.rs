use std::io::Error;
mod solutions;

pub struct Solution {
    sol_part1: fn() -> Result<(), Error>,
    sol_part2: fn() -> Result<(), Error>,
}

pub const SOLUTIONS: [Solution; 1] = [solutions::day_one::DAY_ONE];

fn main() {
    println!("advent of code 2023 - @co_re / thaYt");

    println!("days so far: {}", SOLUTIONS.len());

    for (i, sol) in SOLUTIONS.iter().enumerate() {
        println!("  day {} (parts 1, 2)", i + 1);
    }
}

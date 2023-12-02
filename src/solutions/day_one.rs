use regex::Regex;
use std::io::{self, Error};
use super::super::Solution;

pub const DAY_ONE: Solution = Solution {
    sol_part1: part_one,
    sol_part2: part_two,
};

pub fn part_one() -> Result<(), Error> {
    let mut buffer = String::from("aaaaaaaaaaaaaaaa");
    let exp = Regex::new(r"[A-Za-z]").unwrap();

    let mut i = 0;

    while !buffer.is_empty() {
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let a = exp.replace_all(&buffer, "");

        if buffer == "\r\n" {
            break;
        }

        let b = a.replace("\r\n", "").replace('\n', ""); // i love windows

        // get first and last chars
        let first = b.to_owned().chars().next().unwrap().to_string();
        let last = b.to_owned().chars().last().unwrap().to_string();

        i += format!("{first}{last}").parse::<i32>().unwrap();
    }

    println!("{i}");

    Ok(())
}

pub fn part_two() -> Result<(), Error> {
    let mut buffer = String::from("aaaaaaaaaaaaaaaa");
    let exp = Regex::new(r"[A-Za-z]").unwrap();

    let mut i = 0;

    while !buffer.is_empty() {
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        // probably could be done better
        buffer = buffer
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            .replace("zero", "z0o");

        let a = exp.replace_all(&buffer, "");

        if buffer == "\r\n" {
            break;
        }

        let b = a.replace("\r\n", "");
        // get first and last chars
        let first = b.to_owned().chars().next().unwrap().to_string();
        let last = b.to_owned().chars().last().unwrap().to_string();

        i += format!("{first}{last}").parse::<i32>().unwrap();
    }

    println!("{i}");

    Ok(())
}

use std::fs;
use std::io::{self, Write};

fn puzzle_1() {}

fn puzzle_2() {}

fn main() {
    let input =
        fs::read_to_string("./year/2021/inputs/day1_puzzle1.input").expect("Error reading file.");
    let numbers: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    puzzle_1(numbers);

    let input =
        fs::read_to_string("./year/2021/inputs/day1_puzzle2.input").expect("Error reading file.");
    let mut numbers: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    puzzle_2(numbers);
}

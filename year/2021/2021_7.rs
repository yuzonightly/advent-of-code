use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

fn puzzle_1(input: &Vec<i32>) {
    writeln!(io::stdout(), "Puzzle 1: {}", 1);
}

fn puzzle_2() {
    writeln!(io::stdout(), "Puzzle 2: {}", 1);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day7.input").expect("Error reading file.");
    let split: Vec<i32> = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    puzzle_1(&split);
}

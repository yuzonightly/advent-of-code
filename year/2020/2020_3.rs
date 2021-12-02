use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

fn puzzle_1(input: &Vec<String>) {
    
}

fn main() {
    let input = fs::read_to_string("./year/2020/inputs/day3.input").expect("Error reading file.");
    puzzle_1(&input.lines().collect());
    // puzzle_2(&input);
}

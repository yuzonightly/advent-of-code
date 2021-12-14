use std::fs;
// use std::collections::{HashMap, HashSet};
// use std::error::Error;
// use std::io::{self, Write};
// use std::str::{self, FromStr};

fn puzzle_1(lines: &Vec<&str>) {
    println!("Puzzle 1: {}", 1);
}

fn puzzle_2() {
    println!("Puzzle 2: {}", 1);
}

pub fn run() {
    let input = fs::read_to_string("./year/2020/inputs/day7.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();

    // puzzle_1(&lines);
}

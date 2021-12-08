use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

fn puzzle_1(fishies: &Vec<u32>) {
    let mut map = [0u32; 9];
    for i in 0..9 {
        map[i] = fishies.iter().filter(|&n| *n == i as u32).count() as u32;
    }
    let mut zero = 0;
    for i in 0..80 {
        let temp0 = map[zero];
        zero = (zero + 1) % 7;
        map[(zero + 6) % 7] += map[7];
        map[7] = map[8];
        map[8] = temp0;
    }
    let sum: u32 = map.iter().sum();
    writeln!(io::stdout(), "Puzzle 1: {}", sum);
}

fn puzzle_2(fishies: &Vec<u32>) {
    let mut map = [0u64; 9];
    for i in 0..9 {
        map[i] = fishies.iter().filter(|&n| *n == i as u32).count() as u64;
    }
    let mut zero = 0;
    for i in 0..256 {
        let temp0 = map[zero];
        zero = (zero + 1) % 7;
        map[(zero + 6) % 7] += map[7];
        map[7] = map[8];
        map[8] = temp0;
    }
    let sum: u64 = map.iter().sum();
    writeln!(io::stdout(), "Puzzle 2: {}", sum);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day6.input").expect("Error reading file.");
    let values: Vec<u32> = input
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    puzzle_1(&values);
    puzzle_2(&values);
}

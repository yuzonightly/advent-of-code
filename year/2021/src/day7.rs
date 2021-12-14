use std::fs;
use std::io::{self, Write};

fn puzzle_1(input: &mut Vec<i32>) {
    input.sort();
    let median = input[input.len() / 2 - 1];
    let distances: Vec<i32> = input
        .iter()
        .map(|s| (*s as i32 - median as i32).abs())
        .collect();
    let distance_sum: i32 = distances.iter().sum();
    writeln!(io::stdout(), "Puzzle 1: {}", distance_sum);
}

fn puzzle_2(input: &mut Vec<i32>) {
    let sum: i32 = input.iter().sum();
    let mean: i32 = sum / input.len() as i32;
    let distances: Vec<i32> = input
        .iter()
        .map(|s| {
            let y = (*s - mean).abs();
            y * (y + 1) / 2
        })
        .collect();
    writeln!(io::stdout(), "Puzzle 2: {}", distances.iter().sum::<i32>());
}

/// Fix second part.
pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day7.input").expect("Error reading file.");
    let mut split: Vec<i32> = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    puzzle_1(&mut split);
    puzzle_2(&mut split);
}

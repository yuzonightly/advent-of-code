use std::fs;
use std::io::{self, Write};

fn puzzle_1(numbers: Vec<u32>) {
    let mut increasing = 0;
    let mut previous: u32 = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] > previous {
            increasing += 1;
        }
        previous = numbers[i];
    }
    writeln!(io::stdout(), "Puzzle 1: {}", increasing);
}

fn puzzle_2(mut numbers: Vec<u32>) {
    let mut increasing = 0;
    let mut previous: u32 = numbers[0..3].iter().sum();
    numbers.drain(0..1);
    while numbers.len() >= 3 {
        let sum: u32 = numbers[0..3].iter().sum();
        if sum > previous {
            increasing += 1;
        }
        previous = sum;
        numbers.drain(0..1);
    }
    writeln!(io::stdout(), "Puzzle 2: {}", increasing);
}

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

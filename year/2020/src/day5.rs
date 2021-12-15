use std::fs;
use std::str::{self};

fn puzzle_1(input: &Vec<&str>) {
    let mut row8_plus_col: u32 = 0;
    for line in input {
        let mut lower_row: u32 = 0;
        let mut higher_row: u32 = 127;
        for c in line[0..7].chars() {
            if c == 'B' {
                lower_row = (higher_row - lower_row + 1) / 2 + lower_row;
            } else if c == 'F' {
                higher_row = (higher_row - lower_row) / 2 + lower_row;
            }
        }
        let mut lower_col: u32 = 0;
        let mut higher_col: u32 = 7;
        for c in line[7..line.len()].chars() {
            if c == 'R' {
                lower_col = (higher_col - lower_col + 1) / 2 + lower_col;
            } else if c == 'L' {
                higher_col = (higher_col - lower_col) / 2 + lower_col;
            }
        }
        let temp = (lower_row * 8) + lower_col;
        if temp > row8_plus_col {
            row8_plus_col = temp;
        }
    }
    println!("Puzzle 1: {}", row8_plus_col);
}

fn puzzle_2(input: &Vec<&str>) {
    let mut ids: Vec<u32> = Vec::new();
    for line in input {
        let mut lower_row: u32 = 0;
        let mut higher_row: u32 = 127;
        for c in line[0..7].chars() {
            if c == 'B' {
                lower_row = (higher_row - lower_row + 1) / 2 + lower_row;
            } else if c == 'F' {
                higher_row = (higher_row - lower_row) / 2 + lower_row;
            }
        }
        let mut lower_col: u32 = 0;
        let mut higher_col: u32 = 7;
        for c in line[7..line.len()].chars() {
            if c == 'R' {
                lower_col = (higher_col - lower_col + 1) / 2 + lower_col;
            } else if c == 'L' {
                higher_col = (higher_col - lower_col) / 2 + lower_col;
            }
        }
        if !(lower_row == 0 || higher_row == 127) {
            let temp = (lower_row * 8) + lower_col;
            ids.push(temp);
        }
    }
    ids.sort();
    for i in 0..ids.len() - 1 {
        if ids[i] + 1 != ids[i + 1] {
            println!("Puzzle 2: {:?}", ids[i] + 1);
            std::process::exit(0);
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("./year/2020/inputs/day5.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

use std::collections::HashMap;
use std::fs;
use std::str::{self};

fn puzzle_1(groups: &Vec<&str>) {
    let mut yes = 0;
    for group in groups {
        let members: Vec<&str> = group.split("\n").collect();
        let mut map = HashMap::new();
        for member in members {
            for answer in member.chars() {
                map.insert(answer, 1);
            }
        }
        yes += map.len();
    }
    println!("Puzzle 1: {}", yes);
}

fn puzzle_2(groups: &Vec<&str>) {
    let mut yes = 0;
    for group in groups {
        let members: Vec<&str> = group.split("\n").collect();
        let mut map = HashMap::new();
        for member in &members {
            for answer in member.chars() {
                if let Some(x) = map.get_mut(&answer) {
                    *x = *x + 1;
                } else {
                    map.insert(answer, 1);
                }
            }
        }
        for (_, value) in map {
            if value == members.len() {
                yes += 1;
            }
        }
    }
    println!("Puzzle 2: {}", yes);
}

pub fn run() {
    let input = fs::read_to_string("./year/2020/inputs/day6.input").expect("Error reading file.");
    let groups: Vec<&str> = input.split("\n\n").collect();
    puzzle_1(&groups);
    puzzle_2(&groups);
}

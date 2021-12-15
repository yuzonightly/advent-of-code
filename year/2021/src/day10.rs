use std::collections::HashMap;
use std::fs;
use std::str::{self};

fn puzzle_1(lines: &Vec<&str>) {
    let mut points = 0;
    let open_close: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();
    'next: for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            if open_close.contains_key(&c) {
                stack.push(c);
            } else {
                let expected = match stack.pop() {
                    Some(popped) => *open_close.get(&popped).unwrap(),
                    None => c,
                };
                if expected != c {
                    if c == ')' {
                        points += 3;
                    } else if c == ']' {
                        points += 57;
                    } else if c == '}' {
                        points += 1197;
                    } else if c == '>' {
                        points += 25137;
                    }
                    continue 'next;
                }
            }
        }
    }
    println!("Puzzle 1: {}", points);
}

fn puzzle_2(lines: &Vec<&str>) {
    let open_close: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();
    let mut scores = Vec::new();
    'next: for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            if open_close.contains_key(&c) {
                stack.push(c);
            } else {
                let expected = match stack.pop() {
                    Some(popped) => *open_close.get(&popped).unwrap(),
                    None => c,
                };
                if expected != c {
                    continue 'next;
                }
            }
        }
        let mut score: u64 = 0;
        for i in (0..stack.len()).rev() {
            let c = *open_close.get(&stack[i]).unwrap();
            score *= 5;
            if c == ')' {
                score += 1;
            } else if c == ']' {
                score += 2;
            } else if c == '}' {
                score += 3;
            } else if c == '>' {
                score += 4;
            }
        }
        scores.push(score);
    }
    scores.sort();
    println!("Puzzle 2: {}", scores[scores.len() / 2]);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day10.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

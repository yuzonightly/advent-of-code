use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

struct Lines(Vec<Line>);

#[derive(Clone, Debug)]
struct Line {
    initial: Point,
    end: Point,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Lines {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mut coords: Vec<Line> = Vec::new();
        for line in lines {
            let split: Vec<&str> = line.split(" -> ").collect();
            let pt1_split: Vec<&str> = split[0].split(",").collect();
            let pt2_split: Vec<&str> = split[1].split(",").collect();
            let initial = Point {
                x: pt1_split[0].parse().unwrap(),
                y: pt1_split[1].parse().unwrap(),
            };
            let end = Point {
                x: pt2_split[0].parse().unwrap(),
                y: pt2_split[1].parse().unwrap(),
            };
            coords.push(Line { initial, end });
        }
        Ok(Lines(coords))
    }
}

fn puzzle_1(input: &Vec<Line>) {
    let mut overlaps: HashMap<String, u32> = HashMap::new();
    for line in input {
        let mut temp = line.initial.clone();
        if !(line.initial.x != line.end.x && line.initial.y != line.end.y) {
            while temp != line.end {
                let str: String = format!("{},{}", temp.x, temp.y);
                if let Some(x) = overlaps.get_mut(&str) {
                    *x = *x + 1;
                } else {
                    overlaps.insert(str, 1);
                }
                if temp.x - line.end.x < 0 {
                    temp.x = temp.x + 1;
                } else if temp.x - line.end.x > 0 {
                    temp.x = temp.x - 1;
                }
                if temp.y - line.end.y < 0 {
                    temp.y = temp.y + 1;
                } else if temp.y - line.end.y > 0 {
                    temp.y = temp.y - 1;
                }
            }
            let str: String = format!("{},{}", temp.x, temp.y);
            if let Some(x) = overlaps.get_mut(&str) {
                *x = *x + 1;
            } else {
                overlaps.insert(str, 1);
            }
        }
    }
    let mut overlap_count = 0;
    for (_, value) in overlaps {
        if value >= 2 {
            overlap_count += 1;
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", overlap_count);
}

fn puzzle_2(input: &Vec<Line>) {
    let mut overlaps: HashMap<String, u32> = HashMap::new();
    for line in input {
        let mut temp = line.initial.clone();
        while temp != line.end {
            let str: String = format!("{},{}", temp.x, temp.y);
            if let Some(x) = overlaps.get_mut(&str) {
                *x = *x + 1;
            } else {
                overlaps.insert(str, 1);
            }
            if temp.x - line.end.x < 0 {
                temp.x = temp.x + 1;
            } else if temp.x - line.end.x > 0 {
                temp.x = temp.x - 1;
            }
            if temp.y - line.end.y < 0 {
                temp.y = temp.y + 1;
            } else if temp.y - line.end.y > 0 {
                temp.y = temp.y - 1;
            }
        }
        let str: String = format!("{},{}", temp.x, temp.y);
        if let Some(x) = overlaps.get_mut(&str) {
            *x = *x + 1;
        } else {
            overlaps.insert(str, 1);
        }
    }
    let mut overlap_count = 0;
    for (_, value) in overlaps {
        if value >= 2 {
            overlap_count += 1;
        }
    }
    writeln!(io::stdout(), "Puzzle 2: {}", overlap_count);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day5.input").expect("Error reading file.");
    let lines: Lines = input.parse().unwrap();
    puzzle_1(&lines.0);
    puzzle_2(&lines.0);
}

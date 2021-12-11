use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

fn puzzle_1(input: &Vec<&str>) {
    let mut count = 0;
    let unique: [usize; 4] = [2, 3, 4, 7];
    for instance in input {
        let inp_out: Vec<&str> = instance.split("|").collect::<Vec<&str>>();
        let mut out: Vec<HashSet<char>> = inp_out[1]
            .trim()
            .split(" ")
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        for segments in out {
            if unique.contains(&segments.len()) {
                count += 1;
            }
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {:?}", count);
}

fn puzzle_2(input: &Vec<&str>) {
    let mut count = 0;
    for instance in input {
        let inp_out: Vec<&str> = instance.split("|").collect::<Vec<&str>>();
        let mut inp: Vec<Vec<char>> = inp_out[0]
            .trim()
            .split(" ")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut out: Vec<Vec<char>> = inp_out[1]
            .trim()
            .split(" ")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        inp.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut segment_map = HashMap::new();
        for segments in &inp {
            let mut set: HashSet<i32>;
            if segments.len() == 2 {
                set = vec![2, 5].iter().cloned().collect();
                segment_map.insert(segments[0], set.clone());
                segment_map.insert(segments[1], set.clone());
            } else if segments.len() == 3 {
                set = vec![0].iter().cloned().collect();
                for c in segments {
                    if !segment_map.contains_key(c) {
                        segment_map.insert(*c, set.clone());
                    }
                }
            } else if segments.len() == 4 {
                set = vec![1, 3].iter().cloned().collect();
                for c in segments {
                    if !segment_map.contains_key(c) {
                        segment_map.insert(*c, set.clone());
                    }
                }
            } else if segments.len() == 7 {
                set = vec![4, 6].iter().cloned().collect();
                for c in segments {
                    if !segment_map.contains_key(c) {
                        segment_map.insert(*c, set.clone());
                    }
                }
            }
        }
        let numbers: Vec<Vec<char>> = Vec::new();
        for segments in &inp {
            for (key, value) in segment_map {
                if segments.len() == 6 && !segments.contains(key) {
                    
                }

            } 
                
            )
        }
    }
    // writeln!(io::stdout(), "Puzzle 2: {:?}", segment_map);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day8.input").expect("Error reading file.");
    let lines = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

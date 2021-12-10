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

/**
 * c -> 2, 5
 * f -> 2, 5
 * a -> 0
 * b -> 1, 3
 * d -> 1, 3
 * e -> 4, 6
 * g -> 4, 6
 *
 * 0 = 6
 * 2 = 5
 * 3 = 5
 * 5 = 5
 * 6 = 6
 * 9 = 6
 *
 * 0 -> L d -> b d (if there is only 1 then stop) -> if all but one is in the array, then (d -> 3)
 * update the map
 * 6 -> L c ->
 */

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
        writeln!(io::stdout(), "Puzzle 2: {:?}", inp);
        let mut segment_map = HashMap::new();
        for segments in &inp {
            if segments.len() == 2 {
                segment_map.insert(segments[0], [2, 5]);
                segment_map.insert(segments[1], [2, 5]);
            } else if segments.len() == 3 {
                for c in segments {
                    if !segment_map.contains_key(c) {
                        segment_map.insert(*c, 0);
                    }
                }
            } else if segments.len() == 4 {
                for c in segments {
                    if !segment_map.contains_key(c) {
                        segment_map.insert(*c, [1, 3]);
                    }
                }
            } else if segments.len() == 7 {
                segment_map.insert(segments[0], [2, 5]);
                segment_map.insert(segments[1], [2, 5]);
            }
        }
        writeln!(io::stdout(), "Puzzle 2: {:?}", segment_map);
        for numbers in inp {}
    }
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day8.input").expect("Error reading file.");
    let lines = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

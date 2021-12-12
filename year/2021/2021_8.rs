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
        let out: Vec<HashSet<char>> = inp_out[1]
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
    let mut sum = 0;
    for instance in input {
        let inp_out: Vec<&str> = instance.split("|").collect::<Vec<&str>>();
        let mut inp: Vec<Vec<char>> = inp_out[0]
            .trim()
            .split(" ")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let out: Vec<Vec<char>> = inp_out[1]
            .trim()
            .split(" ")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        inp.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut segment_connections: HashMap<char, Vec<usize>> = HashMap::new();
        let mut segments = &inp[0];
        segment_connections.insert(segments[0], vec![2, 5]);
        segment_connections.insert(segments[1], vec![2, 5]);
        segments = &inp[1];
        for seg in segments {
            if !segment_connections.contains_key(&seg) {
                segment_connections.insert(*seg, vec![0]);
            }
        }
        segments = &inp[2];
        for seg in segments {
            if !segment_connections.contains_key(&seg) {
                segment_connections.insert(*seg, vec![1, 3]);
            }
        }
        segments = &inp[9];
        for seg in segments {
            if !segment_connections.contains_key(&seg) {
                segment_connections.insert(*seg, vec![4, 6]);
            }
        }
        let segment_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        for segments in &inp {
            if segments.len() == 6 {
                let mut missing_segment = 's';
                for letter in &segment_letters {
                    if !segments.contains(&letter) {
                        missing_segment = *letter;
                        break;
                    }
                }
                let mut segment_connections_clone = segment_connections.clone();
                let values = segment_connections.get(&missing_segment).unwrap();
                for (key, value) in &segment_connections {
                    if *values == *value && *key == missing_segment {
                        let s = segment_connections_clone.get_mut(&key).unwrap();
                        if value.contains(&2) {
                            *s = vec![2];
                        } else if value.contains(&1) {
                            *s = vec![3];
                        } else if value.contains(&4) {
                            *s = vec![4];
                        }
                    } else if *values == *value && *key != missing_segment {
                        let s = segment_connections_clone.get_mut(&key).unwrap();
                        if value.contains(&2) {
                            *s = vec![5];
                        } else if value.contains(&1) {
                            *s = vec![1];
                        } else if value.contains(&4) {
                            *s = vec![6];
                        }
                    }
                }
                segment_connections = segment_connections_clone;
            }
        }
        let numbers: Vec<Vec<usize>> = vec![
            vec![0, 1, 2, 4, 5, 6],
            vec![2, 5],
            vec![0, 2, 3, 4, 6],
            vec![0, 2, 3, 5, 6],
            vec![1, 2, 3, 5],
            vec![0, 1, 3, 5, 6],
            vec![0, 1, 3, 4, 5, 6],
            vec![0, 2, 5],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![0, 1, 2, 3, 5, 6],
        ];
        let mut sum_instance = 0;
        for segment in &out {
            let mut coded_number = Vec::new();
            for letter in segment {
                let mut temp = segment_connections.get(&letter).unwrap().clone();
                coded_number.append(&mut temp);
            }
            coded_number.sort();
            let decoded_number = numbers.iter().position(|r| *r == coded_number).unwrap();
            sum_instance = sum_instance * 10 + decoded_number;
        }
        sum += sum_instance;
    }
    writeln!(io::stdout(), "Puzzle 2: {:?}", sum);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day8.input").expect("Error reading file.");
    let lines = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

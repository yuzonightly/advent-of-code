use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

fn puzzle_1(sequence: String, rules: &HashMap<String, String>) {
    let mut pair_count = HashMap::new();
    for (key, value) in rules {
        pair_count.insert(key, 0);
    }
    sequence.as_bytes().windows(2).for_each(|s| {
        let str: String = String::from_utf8_lossy(s).to_string();
        if let Some(s) = pair_count.get_mut(&str) {
            *s += 1;
        }
    });
    // sum for the inserted
    for i in 0..40 {
        let mut pair_count_clone = pair_count.clone();
        for (key, value) in &pair_count {
            if let Some(s) = pair_count_clone.get_mut(*key) {
                *s -= value;
            }
            let key_bytes = key.as_bytes();
            let insert_letter: String = rules.get(*key).unwrap().to_string();
            let first = format!("{}{}", key_bytes[0] as char, insert_letter);
            let second = format!("{}{}", insert_letter, key_bytes[1] as char);
            if let Some(s) = pair_count_clone.get_mut(&first) {
                *s += value;
            }
            if let Some(s) = pair_count_clone.get_mut(&second) {
                *s += value;
            }
        }
        pair_count = pair_count_clone.clone();
    }
    let mut char_counter: HashMap<char, u64> = HashMap::new();
    for (key, value) in &pair_count {
        let bytes = key.as_bytes();
        *char_counter.entry(bytes[0] as char).or_default() += value;
    }
    let most_common = char_counter
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();
    let least_common = char_counter
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();
    println!("Puzzle 1: {:?}", most_common - least_common);
}

fn puzzle_2() {
    println!("Puzzle 2: {:?}", 1);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day14.input").expect("Error reading file.");
    let split_two: Vec<&str> = input.split("\n\n").collect();
    let mut sequence: String = split_two[0].to_owned();
    let rules: HashMap<String, String> = split_two[1]
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(" -> ").collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();
    puzzle_1(sequence, &rules);
}

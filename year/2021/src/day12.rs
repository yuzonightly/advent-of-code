use std::collections::{HashSet};
use std::fs;
use std::io::{self, Write};
use std::str::{self};

fn recursive_single(
    edges: &Vec<(String, String)>,
    current: String,
    visited: &mut Vec<String>,
) -> u32 {
    if current.eq("end") {
        return 1;
    }
    if visited.contains(&current) {
        return 0;
    }
    if current.to_lowercase().eq(&current) {
        visited.push(current.to_string());
    }
    let mut paths = 0;
    for (start, end) in edges {
        if start.eq(&current) {
            paths += recursive_single(&edges, end.to_string(), &mut visited.clone());
        }
    }
    paths
}

fn puzzle_1(input: &Vec<(String, String)>) {
    let mut paths = 0;
    let mut edges = input.clone();
    let mut new_edges = Vec::new();
    for (start, end) in input {
        if start.eq("end") || end.eq("start") {
            new_edges.push((end.to_string(), start.to_string()));
        } else if start.eq("start") || end.eq("end") {
            new_edges.push((start.to_string(), end.to_string()));
        } else {
            new_edges.push((start.to_string(), end.to_string()));
            new_edges.push((end.to_string(), start.to_string()));
        }
    }
    for (start, end) in &new_edges {
        if start.eq("start") {
            paths += recursive_single(&new_edges, end.to_string(), &mut Vec::new());
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", paths);
}

fn recursive_twice(
    edges: &Vec<(String, String)>,
    current: String,
    visited: &mut Vec<String>,
    twice: String,
) -> u32 {
    let count_twice = visited.iter().filter(|&s| s.eq(&twice)).count();
    if current.eq("end") && count_twice == 2 {
        return 1;
    } else if current.eq("end") && count_twice == 1 {
        return 0;
    }
    let count_current = visited.iter().filter(|&s| s.eq(&current)).count();
    if count_current == 2 && current.eq(&twice) {
        return 0;
    } else if count_current == 1 && !current.eq(&twice) {
        return 0;
    }
    if current.to_lowercase().eq(&current) {
        visited.push(current.to_string());
    }
    let mut paths = 0;
    for (start, end) in edges {
        if start.eq(&current) {
            paths += recursive_twice(
                &edges,
                end.to_string(),
                &mut visited.clone(),
                twice.to_string(),
            );
        }
    }
    paths
}

fn puzzle_2(input: &Vec<(String, String)>) {
    let mut new_edges = Vec::new();
    let mut every_lowercase_edge = HashSet::new();
    for (start, end) in input {
        if start.eq("end") || end.eq("start") {
            new_edges.push((end.to_string(), start.to_string()));
        } else if start.eq("start") || end.eq("end") {
            new_edges.push((start.to_string(), end.to_string()));
        } else {
            new_edges.push((start.to_string(), end.to_string()));
            new_edges.push((end.to_string(), start.to_string()));
            if start.eq(&start.to_lowercase()) {
                every_lowercase_edge.insert(start);
            }
            if end.eq(&end.to_lowercase()) {
                every_lowercase_edge.insert(end);
            }
        }
    }
    let mut paths = 0;
    for twice in every_lowercase_edge {
        for (start, end) in &new_edges {
            if start.eq("start") {
                paths += recursive_twice(
                    &new_edges,
                    end.to_string(),
                    &mut Vec::new(),
                    twice.to_string(),
                );
            }
        }
    }
    for (start, end) in &new_edges {
        if start.eq("start") {
            paths += recursive_single(&new_edges, end.to_string(), &mut Vec::new());
        }
    }
    writeln!(io::stdout(), "Puzzle 2: {}", paths);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day12.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    let edges: Vec<(String, String)> = lines
        .into_iter()
        .map(|s| {
            let split: Vec<&str> = s.split("-").collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();
    puzzle_1(&edges);
    puzzle_2(&edges);
}

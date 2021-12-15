use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::{self};

fn puzzle_1(mut coordinates: HashSet<(u32, u32)>, folds: &Vec<(char, u32)>) {
    let (fold_coord, fold_index) = folds[0];
    let mut coordinates_clone = coordinates.clone();
    for (x, y) in coordinates.into_iter() {
        let sub = fold_index * 2;
        if fold_coord == 'x' {
            if x > fold_index {
                let new_x = sub - x;
                coordinates_clone.remove(&(x, y));
                coordinates_clone.insert((new_x, y));
            } else if x == fold_index {
                coordinates_clone.remove(&(x, y));
            }
        } else if fold_coord == 'y' {
            if y > fold_index {
                let new_y = sub - y;
                coordinates_clone.remove(&(x, y));
                coordinates_clone.insert((x, new_y));
            } else if y == fold_index {
                coordinates_clone.remove(&(x, y));
            }
        }
    }
    coordinates = coordinates_clone;
    println!("Puzzle 1: {:?}", coordinates.len());
}

fn puzzle_2(mut coordinates: HashSet<(u32, u32)>, folds: &Vec<(char, u32)>) {
    for (fold_coord, fold_index) in folds.into_iter() {
        let mut coordinates_clone = coordinates.clone();
        for (x, y) in coordinates.into_iter() {
            let sub = *fold_index * 2;
            if *fold_coord == 'x' {
                if x > *fold_index {
                    let new_x = sub - x;
                    coordinates_clone.remove(&(x, y));
                    coordinates_clone.insert((new_x, y));
                } else if x == *fold_index {
                    coordinates_clone.remove(&(x, y));
                }
            } else if *fold_coord == 'y' {
                if y > *fold_index {
                    let new_y = sub - y;
                    coordinates_clone.remove(&(x, y));
                    coordinates_clone.insert((x, new_y));
                } else if y == *fold_index {
                    coordinates_clone.remove(&(x, y));
                }
            }
        }
        coordinates = coordinates_clone;
    }
    println!("{}", "Puzzle 2:");
    let mut print: HashMap<usize, Vec<char>> = HashMap::new();
    for (x, y) in &coordinates {
        if let Some(s) = print.get_mut(&(*y as usize)) {
            let line_length = s.len() as u32;
            if *x + 1 > line_length {
                let insert_length = *x + 1 - line_length;
                let mut chars = Vec::new();
                for i in 0..insert_length {
                    if i == insert_length - 1 {
                        chars.push('#');
                    } else {
                        chars.push(' ');
                    }
                }
                s.append(&mut chars);
            } else {
                s[*x as usize] = '#';
            }
        } else {
            let mut chars = Vec::new();
            for i in 0..*x + 1 {
                if i == *x {
                    chars.push('#');
                } else {
                    chars.push(' ');
                }
            }
            print.insert(*y as usize, chars);
        }
    }
    for i in 0..print.len() {
        let chars = print.get(&i).unwrap();
        let string: String = chars.into_iter().collect();
        println!("{}", string);
    }
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day13.input").expect("Error reading file.");
    let coord_fold: Vec<&str> = input.split("\n\n").collect();
    let coord: HashSet<(u32, u32)> = coord_fold[0]
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(",").collect();
            let x = split[0].parse::<u32>().unwrap();
            let y = split[1].parse::<u32>().unwrap();
            (x, y)
        })
        .collect();
    let folds: Vec<(char, u32)> = coord_fold[1]
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split_whitespace().collect();
            let xy: Vec<&str> = split[2].split("=").collect();
            (
                xy[0].parse::<char>().unwrap(),
                xy[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    let coord_clone = coord.clone();
    puzzle_1(coord, &folds);
    puzzle_2(coord_clone, &folds);
}

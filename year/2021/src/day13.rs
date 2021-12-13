use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

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
    writeln!(io::stdout(), "Puzzle 1: {:?}", coordinates.len());
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

    let root = BitMapBackend::new("./year/2021/day13_code.png", (500, 500)).into_drawing_area();
    root.fill(&RGBColor(255, 255, 255));
    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..50f32,
        0f32..50f32,
        (0..500, 0..500),
    ));
    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled());
    };
    for (x, y) in &coordinates {
        root.draw(&dot_and_label(*x as f32, *y as f32));
    }
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day13.input").expect("Error reading file.");
    let coord_fold: Vec<&str> = input.split("\n\n").collect();
    let mut coord: HashSet<(u32, u32)> = coord_fold[0]
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
    let mut coord_clone = coord.clone();
    puzzle_1(coord, &folds);
    puzzle_2(coord_clone, &folds);
}

use std::collections::HashSet;
use std::fs;
use std::str::{self};

fn find_edges(i: usize, j: usize, rows: usize, cols: usize) -> (usize, usize, usize, usize) {
    let (x1, y1) = (
        if (i as i32 - 1) < 0 { 0 } else { i - 1 },
        if (j as i32 - 1) < 0 { 0 } else { j - 1 },
    );
    let (x2, y2) = ((i + 1) - ((i + 1) / rows), (j + 1) - ((j + 1) / cols));
    (x1, y1, x2, y2)
}

fn puzzle_1(matrix: &Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut sum = 0;
    for i in 0..rows {
        'next: for j in 0..cols {
            let (x1, y1, x2, y2) = find_edges(i, j, rows, cols);
            for adj_i in x1..x2 + 1 {
                for adj_j in y1..y2 + 1 {
                    if matrix[adj_i][adj_j] < matrix[i][j] {
                        continue 'next;
                    }
                }
            }
            sum += matrix[i][j] + 1;
        }
    }
    println!("Puzzle 1: {}", sum);
}

fn puzzle_2(matrix: &Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut basins = Vec::new();
    for i in 0..rows {
        'next: for j in 0..cols {
            let (x1, y1, x2, y2) = find_edges(i, j, rows, cols);
            for adj_i in x1..x2 + 1 {
                for adj_j in y1..y2 + 1 {
                    if matrix[adj_i][adj_j] < matrix[i][j] {
                        continue 'next;
                    }
                }
            }
            let mut to_explore = vec![(i, j)];
            let mut explored = HashSet::new();
            while to_explore.len() > 0 {
                let (x, y) = to_explore[0];
                let (x1, y1, x2, y2) = find_edges(x, y, rows, cols);
                for adj_i in x1..x2 + 1 {
                    if matrix[adj_i][y] != 9 && matrix[adj_i][y] > matrix[x][y] {
                        to_explore.push((adj_i, y));
                    }
                }
                for adj_j in y1..y2 + 1 {
                    if matrix[x][adj_j] != 9 && matrix[x][adj_j] > matrix[x][y] {
                        to_explore.push((x, adj_j));
                    }
                }
                to_explore.remove(0);
                explored.insert((x, y));
            }
            basins.push(explored.len());
        }
    }
    basins.sort_by(|a, b| b.cmp(a));
    println!("Puzzle 2: {}", basins[0] * basins[1] * basins[2]);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day9.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    let matrix: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();
    puzzle_1(&matrix);
    puzzle_2(&matrix);
}

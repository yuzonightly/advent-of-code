use std::fs;
use std::io::{self, Write};
use std::str::{self};

fn find_edges(i: usize, j: usize, rows: usize, cols: usize) -> (usize, usize, usize, usize) {
    let (x1, y1) = (
        if (i as i32 - 1) < 0 { 0 } else { i - 1 },
        if (j as i32 - 1) < 0 { 0 } else { j - 1 },
    );
    let (x2, y2) = ((i + 1) - ((i + 1) / rows), (j + 1) - ((j + 1) / cols));
    (x1, y1, x2, y2)
}

fn step(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            matrix[i][j] += 1;
        }
    }
}

fn puzzle_1(matrix: &Vec<Vec<i32>>) {
    let mut input = matrix.clone();
    let rows = input.len();
    let cols = input[0].len();
    let mut flash_set: Vec<(usize, usize)> = Vec::new();
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_set = Vec::new();
        step(&mut input);
        for i in 0..rows {
            for j in 0..cols {
                if input[i][j] > 9 {
                    flash_set.push((i, j));
                }
            }
        }
        while !flash_set.is_empty() {
            flash_count += 1;
            let (x, y) = flash_set.pop().unwrap();
            input[x][y] = 0;
            let (x1, y1, x2, y2) = find_edges(x, y, rows, cols);
            for adj_x in x1..x2 + 1 {
                for adj_y in y1..y2 + 1 {
                    if input[adj_x][adj_y] != 0 {
                        input[adj_x][adj_y] += 1;
                        if input[adj_x][adj_y] > 9 {
                            if !flash_set.contains(&(adj_x, adj_y)) {
                                flash_set.push((adj_x, adj_y));
                            }
                        }
                    }
                }
            }
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", flash_count);
}

fn puzzle_2(matrix: &Vec<Vec<i32>>) {
    let mut input = matrix.clone();
    let rows = input.len();
    let cols = input[0].len();
    let mut flash_set: Vec<(usize, usize)> = Vec::new();
    let mut flash_count = 0;
    let mut loop_index = 0;
    loop {
        loop_index += 1;
        flash_set = Vec::new();
        step(&mut input);
        for i in 0..rows {
            for j in 0..cols {
                if input[i][j] > 9 {
                    flash_set.push((i, j));
                }
            }
        }
        let temp = flash_count;
        while !flash_set.is_empty() {
            flash_count += 1;
            let (x, y) = flash_set.pop().unwrap();
            input[x][y] = 0;
            let (x1, y1, x2, y2) = find_edges(x, y, rows, cols);
            for adj_x in x1..x2 + 1 {
                for adj_y in y1..y2 + 1 {
                    if input[adj_x][adj_y] != 0 {
                        input[adj_x][adj_y] += 1;
                        if input[adj_x][adj_y] > 9 {
                            if !flash_set.contains(&(adj_x, adj_y)) {
                                flash_set.push((adj_x, adj_y));
                            }
                        }
                    }
                }
            }
        }
        if flash_count - temp == rows * cols {
            break;
        }
    }
    writeln!(io::stdout(), "Puzzle 2: {}", loop_index);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day11.input").expect("Error reading file.");
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

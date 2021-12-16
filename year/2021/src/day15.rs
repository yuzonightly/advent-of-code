use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::str::{self};

fn find_edges(i: usize, j: usize, row_len: usize, col_len: usize) -> (usize, usize, usize, usize) {
    let (x1, y1) = (
        if (i as i32 - 1) < 0 { 0 } else { i - 1 },
        if (j as i32 - 1) < 0 { 0 } else { j - 1 },
    );
    let (x2, y2) = ((i + 1) - ((i + 1) / row_len), (j + 1) - ((j + 1) / col_len));
    (x1, y1, x2, y2)
}

fn puzzle_1(matrix: &Vec<Vec<u32>>) {
    let row_len: usize = matrix.len();
    let col_len: usize = matrix[0].len();
    let row_len5: usize = row_len;
    let col_len5: usize = col_len;
    let goal = (row_len5 - 1, col_len5 - 1);
    let mut distances: Vec<Vec<u32>> = vec![vec![u32::MAX; col_len5]; row_len5];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0u32, 0usize, 0usize)));
    while let Some(Reverse((risk, i, j))) = queue.pop() {
        let block = ((i / row_len) + (j / col_len)) as u32;
        let original_risk = matrix[i % row_len][j % col_len];
        let temp = ((original_risk + block - 1) % 9) + 1;
        let current_risk: u32 = if temp == 0 {
            temp + risk + 1
        } else {
            temp + risk
        };
        if current_risk < distances[i][j] {
            distances[i][j] = current_risk;
        } else {
            continue;
        }
        if goal == (i, j) {
            break;
        }
        let (i1, j1, i2, j2) = find_edges(i, j, row_len5, col_len5);
        let adjacents = vec![(i1, j), (i2, j), (i, j1), (i, j2)];
        for (x, y) in adjacents {
            if !(x == i && y == j) {
                queue.push(Reverse((current_risk, x, y)));
            }
        }
    }
    println!(
        "Puzzle 1: {:?}",
        distances[row_len5 - 1][col_len5 - 1] - distances[0][0]
    );
}

fn puzzle_2(matrix: &Vec<Vec<u32>>) {
    let row_len: usize = matrix.len();
    let col_len: usize = matrix[0].len();
    let row_len5: usize = row_len * 5;
    let col_len5: usize = col_len * 5;
    let goal = (row_len5 - 1, col_len5 - 1);
    let mut distances: Vec<Vec<u32>> = vec![vec![u32::MAX; col_len5]; row_len5];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0u32, 0usize, 0usize)));
    while let Some(Reverse((risk, i, j))) = queue.pop() {
        let block = ((i / row_len) + (j / col_len)) as u32;
        let original_risk = matrix[i % row_len][j % col_len];
        let temp = ((original_risk + block - 1) % 9) + 1;
        let current_risk: u32 = if temp == 0 {
            temp + risk + 1
        } else {
            temp + risk
        };
        if current_risk < distances[i][j] {
            distances[i][j] = current_risk;
        } else {
            continue;
        }
        if goal == (i, j) {
            break;
        }
        let (i1, j1, i2, j2) = find_edges(i, j, row_len5, col_len5);
        let adjacents = vec![(i1, j), (i2, j), (i, j1), (i, j2)];
        for (x, y) in adjacents {
            if !(x == i && y == j) {
                queue.push(Reverse((current_risk, x, y)));
            }
        }
    }
    println!(
        "Puzzle 2: {:?}",
        distances[row_len5 - 1][col_len5 - 1] - distances[0][0]
    );
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day15.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    let matrix: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|s| s.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    puzzle_1(&matrix);
    puzzle_2(&matrix);
}

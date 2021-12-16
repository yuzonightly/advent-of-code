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
    let mut distances: Vec<Vec<Option<u32>>> = vec![vec![None::<u32>; col_len5]; row_len5];
    let mut queue: Vec<((usize, usize), u32)> = vec![((0, 0), 0)];
    'next: while queue.len() > 0 {
        let ((_, _), min_risk) = queue.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let min_risk_index = queue.iter().position(|&r| r.1 == *min_risk).unwrap();
        let ((i, j), risk) = queue.remove(min_risk_index);
        let block = ((i / row_len) + (j / col_len)) as u32;
        let original_value = matrix[i % row_len][j % col_len];
        let temp = ((original_value + block - 1) % 9) + 1;
        let current_risk: u32 = if temp == 0 {
            temp + risk + 1
        } else {
            temp + risk
        };
        if distances[i][j] == None || distances[i][j].unwrap() > current_risk {
            distances[i][j] = Some(current_risk);
        } else {
            continue 'next;
        }
        if i == row_len5 - 1 && j == col_len5 - 1 {
            break 'next;
        }
        let (i1, j1, i2, j2) = find_edges(i, j, row_len5, col_len5);
        let adjacents = vec![(i1, j), (i2, j), (i, j1), (i, j2)];
        for (x, y) in adjacents {
            if !(x == i && y == j) {
                queue.push(((x, y), distances[i][j].unwrap()));
            }
        }
    }
    println!(
        "Puzzle 1: {:?}",
        distances[row_len5 - 1][col_len5 - 1].unwrap() - distances[0][0].unwrap()
    );
}

fn puzzle_2(matrix: &Vec<Vec<u32>>) {
    let row_len: usize = matrix.len();
    let col_len: usize = matrix[0].len();
    let row_len5: usize = row_len * 5;
    let col_len5: usize = col_len * 5;
    let mut distances: Vec<Vec<Option<u32>>> = vec![vec![None::<u32>; col_len5]; row_len5];
    let mut queue: Vec<((usize, usize), u32)> = vec![((0, 0), 0)];
    'next: while queue.len() > 0 {
        let ((_, _), min_risk) = queue.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let min_risk_index = queue.iter().position(|&r| r.1 == *min_risk).unwrap();
        let ((i, j), risk) = queue.remove(min_risk_index);
        let block = ((i / row_len) + (j / col_len)) as u32;
        let original_value = matrix[i % row_len][j % col_len];
        let temp = ((original_value + block - 1) % 9) + 1;
        let current_risk: u32 = if temp == 0 {
            temp + risk + 1
        } else {
            temp + risk
        };
        if distances[i][j] == None || distances[i][j].unwrap() > current_risk {
            distances[i][j] = Some(current_risk);
        } else {
            continue 'next;
        }
        let (i1, j1, i2, j2) = find_edges(i, j, row_len5, col_len5);
        let adjacents = vec![(i1, j), (i2, j), (i, j1), (i, j2)];
        for (x, y) in adjacents {
            if !(x == i && y == j) {
                queue.push(((x, y), distances[i][j].unwrap()));
            }
        }
    }
    println!(
        "Puzzle 2: {:?}",
        distances[row_len5 - 1][col_len5 - 1].unwrap() - distances[0][0].unwrap()
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

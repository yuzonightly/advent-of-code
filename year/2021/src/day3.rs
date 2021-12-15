use std::fs;

fn puzzle_1(data: &Vec<&str>) {
    let line_len = data[0].len();
    let mut zeros: Vec<u32> = vec![0; line_len];
    for line in data {
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap() == '0' {
                zeros[i] += 1;
            }
        }
    }
    let input_len = data.len();
    let mut bits: String = "".to_string();
    for zero in zeros {
        if zero > input_len as u32 - zero {
            bits += "0";
        } else {
            bits += "1";
        }
    }
    let ones = (2u32.pow(line_len as u32)) - 1;
    let decimal = u32::from_str_radix(&bits, 2).unwrap();
    let inverted = ones - decimal;
    println!("Puzzle 1: {}", decimal * inverted);
}

fn puzzle_2(data: &Vec<&str>) {
    let line_len = data[0].len();
    let mut data_clone_fewer = data.clone();
    let mut data_clone_more = data.clone();
    let mut position: usize = 0;
    while data_clone_fewer.len() as u32 > 1 {
        let mut zeros: Vec<&str> = Vec::new();
        let mut ones: Vec<&str> = Vec::new();
        for &line in &data_clone_fewer {
            if line.chars().nth(position).unwrap() == '0' {
                zeros.push(line.clone());
            } else {
                ones.push(line.clone());
            }
        }
        if zeros.len() <= ones.len() {
            data_clone_fewer = zeros.clone();
        } else {
            data_clone_fewer = ones.clone();
        }
        position = (position + 1usize) % line_len;
    }

    position = 0;
    while data_clone_more.len() as u32 > 1 {
        let mut zeros: Vec<&str> = Vec::new();
        let mut ones: Vec<&str> = Vec::new();
        for &line in &data_clone_more {
            if line.chars().nth(position).unwrap() == '0' {
                zeros.push(line.clone());
            } else {
                ones.push(line.clone());
            }
        }
        if zeros.len() > ones.len() {
            data_clone_more = zeros.clone();
        } else {
            data_clone_more = ones.clone();
        }
        position = (position + 1usize) % line_len;
    }
    let fewer = u32::from_str_radix(&data_clone_fewer[0], 2).unwrap();
    let more = u32::from_str_radix(&data_clone_more[0], 2).unwrap();
    println!("Puzzle 2: {}", fewer * more);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day3.input").expect("Error reading file.");
    let lines: Vec<&str> = input.lines().collect();
    puzzle_1(&lines);
    puzzle_2(&lines);
}

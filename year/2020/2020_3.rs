use std::fs;
use std::io::{self, Write};

struct Config {
    right: usize,
    down: usize,
}

fn is_tree(thing: char) -> bool {
    if thing == '#' {
        true
    } else {
        false
    }
}

fn puzzle_1(map: &Vec<&str>) {
    let config: Config = Config { right: 3, down: 1 };
    let mut lineno = 0;
    let mut trees = 0;
    for line in map {
        let line_len = line.len();
        let index = {
            let idx = lineno * config.right;
            if idx > line_len - 1 {
                idx % line_len
            } else {
                idx
            }
        };
        if is_tree(line.chars().nth(index).unwrap()) {
            trees += 1;
        }
        lineno += config.down;
    }
    writeln!(io::stdout(), "Puzzle 1: {}", trees);
}

fn puzzle_2(map: &Vec<&str>) {
    let configs: Vec<Config> = vec![
        Config { right: 1, down: 1 },
        Config { right: 3, down: 1 },
        Config { right: 5, down: 1 },
        Config { right: 7, down: 1 },
        Config { right: 1, down: 2 },
    ];
    let mut tree_mult: u64 = 1;
    for conf in configs {
        let mut trees = 0;
        let mut lineno = 0;
        for i in (0..map.len()).step_by(conf.down) {
            let line = map.get(i).unwrap();
            let line_len = line.len();
            let index = {
                let idx = lineno * conf.right;
                if idx > line_len - 1 {
                    idx % line_len
                } else {
                    idx
                }
            };
            if is_tree(line.chars().nth(index).unwrap()) {
                trees += 1;
            }
            lineno += 1;
        }
        tree_mult *= trees;
    }
    writeln!(io::stdout(), "Puzzle 2: {}", tree_mult);
}

fn main() {
    let input = fs::read_to_string("./year/2020/inputs/day3.input").expect("Error reading file.");
    let map: Vec<&str> = input.lines().collect();
    puzzle_1(&map);
    puzzle_2(&map);
}

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

#[derive(Clone, Debug)]
struct Submarine {
    position: Position,
    aim: i32,
}

#[derive(Clone, Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

#[derive(Clone, Debug)]
enum Action {
    Forward { value: i32 },
    Up { value: i32 },
    Down { value: i32 },
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split: Vec<&str> = s.trim().split_whitespace().collect();
        let act: String = split[0].to_owned();
        let value: i32 = split[1].parse()?;
        let action: Action = match act.as_str() {
            "forward" => Action::Forward { value },
            "up" => Action::Up { value },
            "down" => Action::Down { value },
            _ => {
                panic!("FromStr Action error.");
            }
        };
        Ok(action)
    }
}

impl Submarine {
    fn perform_action(&mut self, action: &Action) {
        match action {
            Action::Forward { value } => {
                self.position.horizontal += value;
            }
            Action::Up { value } => {
                self.position.depth -= value;
            }
            Action::Down { value } => {
                self.position.depth += value;
            }
        }
    }

    fn perform_correct_action(&mut self, action: &Action) {
        match action {
            Action::Forward { value } => {
                self.position.horizontal += value;
                self.position.depth += value * self.aim;
            }
            Action::Up { value } => {
                self.aim -= value;
            }
            Action::Down { value } => {
                self.aim += value;
            }
        }
    }
}

fn puzzle_1(submarine: &mut Submarine, actions: &Vec<Action>) {
    for act in actions {
        submarine.perform_action(&act);
    }
    writeln!(
        io::stdout(),
        "Puzzle 1: {} * {} = {}",
        submarine.position.horizontal,
        submarine.position.depth,
        submarine.position.depth * submarine.position.horizontal
    );
}

fn puzzle_2(submarine: &mut Submarine, actions: &Vec<Action>) {
    for act in actions {
        submarine.perform_correct_action(&act);
    }
    writeln!(
        io::stdout(),
        "Puzzle 2: {} * {} = {}",
        submarine.position.horizontal,
        submarine.position.depth,
        submarine.position.depth * submarine.position.horizontal
    );
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day2.input").expect("Error reading file.");
    let actions: Vec<Action> = input
        .lines()
        .map(|a| a.parse::<Action>().unwrap())
        .collect();
    let mut submarine = Submarine {
        position: Position {
            horizontal: 0,
            depth: 0,
        },
        aim: 0,
    };
    let mut new_submarine = submarine.clone();
    puzzle_1(&mut submarine, &actions);
    puzzle_2(&mut new_submarine, &actions);
}

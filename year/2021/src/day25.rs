use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

// #[derive(Clone, Debug, Eq, PartialEq)]
// struct Action {
//     act: i32,
// }

// impl FromStr for Action {
//     type Err = Box<dyn Error>;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }

fn puzzle_1() {
    writeln!(io::stdout(), "Puzzle 1: {:?}", 1);
}

fn puzzle_2() {
    writeln!(io::stdout(), "Puzzle 2: {:?}", 1);
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day25.input").expect("Error reading file.");
    puzzle_1();
}

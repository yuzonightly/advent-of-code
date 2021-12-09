use std::collections::HashMap;
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

fn puzzle_1(input: &Vec<&str>) {
    for instance in input {
        let inp_out: Vec<&str> = instance.split("|").collect::<Vec<&str>>();
        let inp: Vec<&str> = inp_out[0].trim().split(" ").collect();
        let out: Vec<&str> = inp_out[1].trim().split(" ").collect();
        
        writeln!(io::stdout(), "Puzzle 1: {:?} {:?}", inp, out);
    }
}

fn puzzle_2() {
    writeln!(io::stdout(), "Puzzle 2: {}", 1);
}

fn main() {
    let input = fs::read_to_string("./year/2021/inputs/day8.input").expect("Error reading file.");
    let lines = input.lines().collect();
    puzzle_1(&lines);
}

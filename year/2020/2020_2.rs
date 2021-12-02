use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

#[derive(Clone, Debug)]
struct Passwords(Vec<Password>);

#[derive(Clone, Debug)]
struct Password {
    letter_repetition: Range,
    letter: char,
    password: String,
}

// a.k.a positions
#[derive(Clone, Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl FromStr for Passwords {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let passwords: Vec<Password> = lines
            .into_iter()
            .map(|p| p.parse::<Password>().unwrap())
            .collect();
        Ok(Passwords(passwords))
    }
}

impl FromStr for Password {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter_repetition: Range = s.parse().unwrap();
        let mut split: Vec<&str> = s.trim().split_whitespace().collect();
        let letter: char = split[1].chars().nth(0).unwrap();
        let password: String = split[2].parse().unwrap();
        Ok(Password {
            letter_repetition,
            letter,
            password,
        })
    }
}

impl FromStr for Range {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split: Vec<&str> = s.trim().split_whitespace().collect();
        split = split[0].split("-").collect();
        let min: u32 = split[0].parse()?;
        let max: u32 = split[1].parse()?;
        Ok(Range { min, max })
    }
}

fn puzzle_1(passwords: &Vec<Password>) {
    let mut valid = 0;
    for pass in passwords {
        let occurrences = pass.password.matches(pass.letter).count() as u32;
        if pass.letter_repetition.min <= occurrences && pass.letter_repetition.max >= occurrences {
            valid += 1;
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", valid);
}

fn puzzle_2(passwords: &Vec<Password>) {
    let mut valid = 0;
    for pass in passwords {
        if (pass
            .password
            .chars()
            .nth(pass.letter_repetition.min as usize - 1)
            .unwrap()
            == pass.letter)
            ^ (pass
                .password
                .chars()
                .nth(pass.letter_repetition.max as usize - 1)
                .unwrap()
                == pass.letter)
        {
            valid += 1;
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", valid);
}

fn main() {
    let input = fs::read_to_string("./year/2020/inputs/day2.input").expect("Error reading file.");
    let pass: Passwords = input.parse().unwrap();
    puzzle_1(&pass.0);
    puzzle_2(&pass.0);
}

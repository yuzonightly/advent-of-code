use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};

#[derive(Clone, Debug)]
struct Passports(Vec<Passport>);

#[derive(Clone, Debug)]
struct Passport {
    info: HashMap<String, String>,
}

impl FromStr for Passports {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let passports: Vec<&str> = s.split("\n\n").collect();
        let mut vec: Vec<Passport> = Vec::new();
        for pass in &passports {
            let pass: Vec<&str> = pass.split_whitespace().collect();
            let mut map: HashMap<String, String> = HashMap::new();
            for data in pass {
                let split: Vec<&str> = data.split(":").collect();
                map.insert(split[0].to_owned(), split[1].to_owned());
            }
            vec.push(Passport { info: map });
        }
        Ok(Passports(vec))
    }
}

fn puzzle_1(passports: &Vec<Passport>) {
    let to_find = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports = 0;
    for passport in passports {
        let mut correctness = 0;
        for &data in &to_find {
            match passport.info.get(data) {
                Some(key) => {
                    valid_passports -= 1;
                    break;
                }
                _ => {}
            }
        }
        if correctness == 7 {
            valid_passports += 1;
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", valid_passports);
}

fn puzzle_2(passports: &Vec<Passport>) {
    let to_find = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports = passports.len();
    for passport in passports {
        for &data in &to_find {
            match passport.info.get(data) {
                Some(value) => {
                    // valid_passports -= 1;
                    break;
                }
                _ => {}
            }
        }
    }
    writeln!(io::stdout(), "Puzzle 2: {}", valid_passports);
}

fn main() {
    let input = fs::read_to_string("./year/2020/inputs/day4.input").expect("Error reading file.");
    let passports: Passports = input.parse().unwrap();
    puzzle_1(&passports.0);
    puzzle_2(&passports.0);

}

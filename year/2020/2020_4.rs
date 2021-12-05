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
    let mut valid_passports = passports.len();
    for passport in passports {
        for &data in &to_find {
            match passport.info.get(data) {
                None => {
                    valid_passports -= 1;
                    break;
                }
                _ => {}
            }
        }
    }
    writeln!(io::stdout(), "Puzzle 1: {}", valid_passports);
}

fn puzzle_2(passports: &Vec<Passport>) {
    let to_find = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports = passports.len();
    for passport in passports {
        'passport: for &data in &to_find {
            let value = match passport.info.get(data) {
                Some(s) => s,
                _ => {
                    valid_passports -= 1;
                    break 'passport;
                }
            };
            match data {
                "byr" => {
                    let parse = value.parse::<u32>();
                    if parse.is_ok() {
                        let number = parse.unwrap();
                        if number < 1920 || number > 2002 {
                            valid_passports -= 1;
                            break 'passport;
                        }
                    } else {
                        valid_passports -= 1;
                        break 'passport;
                    }
                }
                "iyr" => {
                    let parse = value.parse::<u32>();
                    if parse.is_ok() {
                        let number = parse.unwrap();
                        if number < 2010 || number > 2020 {
                            valid_passports -= 1;
                            break 'passport;
                        }
                    } else {
                        valid_passports -= 1;
                        break 'passport;
                    }
                }
                "eyr" => {
                    let parse = value.parse::<u32>();
                    if parse.is_ok() {
                        let number = parse.unwrap();
                        if number < 2020 || number > 2030 {
                            valid_passports -= 1;
                            break 'passport;
                        }
                    } else {
                        valid_passports -= 1;
                        break 'passport;
                    }
                }
                "hgt" => {
                    let parse = value[0..value.len() - 2].parse::<u32>();
                    if parse.is_err() {
                        valid_passports -= 1;
                        break 'passport;
                    }
                    let number = parse.unwrap();
                    match &value[value.len() - 2..] {
                        "cm" => {
                            if number < 150 || number > 193 {
                                valid_passports -= 1;
                                break 'passport;
                            }
                        }
                        "in" => {
                            if number < 59 || number > 76 {
                                valid_passports -= 1;
                                break 'passport;
                            }
                        }
                        _ => {
                            valid_passports -= 1;
                            break 'passport;
                        }
                    }
                }
                "hcl" => {
                    if value.chars().nth(0).unwrap() != '#' {
                        valid_passports -= 1;
                        break 'passport;
                    }
                    for c in value[1..].chars() {
                        if !c.is_numeric() {
                            if (c as u32) < 97 || (c as u32) > 102 {
                                valid_passports -= 1;
                                break 'passport;
                            }
                        }
                    }
                }
                "ecl" => {
                    let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if !colors.contains(&value.as_str()) {
                        valid_passports -= 1;
                        break 'passport;
                    }
                }
                "pid" => {
                    if value.parse::<u32>().is_err() || value.len() != 9 {
                        valid_passports -= 1;
                        break 'passport;
                    }
                }
                _ => {}
            };
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

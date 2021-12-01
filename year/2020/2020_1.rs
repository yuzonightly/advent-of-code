use std::fs;
use std::io::{self, Write};

fn puzzle_1(numbers: &Vec<u32>) {
    for n1 in numbers {
        for n2 in numbers {
            if n1 + n2 == 2020 {
                writeln!(io::stdout(), "Puzzle 1: {} * {} = {}", n1, n2, n1 * n2);
            }
        }
    }
}

fn puzzle_2(numbers: &Vec<u32>) {
    for n1 in numbers {
        for n2 in numbers {
            for n3 in numbers {
                if (n1 + n2 + n3) == 2020 {
                    writeln!(
                        io::stdout(),
                        "Puzzle 1: {} * {} * {} = {}",
                        n1,
                        n2,
                        n3,
                        n1 * n2 * n3
                    );
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./year/2020/inputs/day1.input").expect("Error reading file.");
    let numbers: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    puzzle_1(&numbers);
    puzzle_2(&numbers);
}

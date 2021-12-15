use std::fs;

fn puzzle_1(numbers: &Vec<u32>) {
    'end: for n1 in numbers {
        for n2 in numbers {
            if n1 + n2 == 2020 {
                println!("Puzzle 1: {} * {} = {}", n1, n2, n1 * n2);
                break 'end;
            }
        }
    }
}

fn puzzle_2(numbers: &Vec<u32>) {
    'end: for n1 in numbers {
        for n2 in numbers {
            for n3 in numbers {
                if (n1 + n2 + n3) == 2020 {
                    println!("Puzzle 2: {} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
                    break 'end;
                }
            }
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("./year/2020/inputs/day1.input").expect("Error reading file.");
    let numbers: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    puzzle_1(&numbers);
    puzzle_2(&numbers);
}

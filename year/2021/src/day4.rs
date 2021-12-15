use std::error::Error;
use std::fs;
use std::str::{self, FromStr};

#[derive(Clone, Debug)]
struct Game {
    calls: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board(Vec<Vec<i32>>);

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n\n").collect();
        let calls: Vec<i32> = lines[0]
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut boards: Vec<Board> = Vec::new();
        for board in &lines[1..] {
            let board_rows: Vec<&str> = board.lines().collect();
            let mut bingo_board: Vec<Vec<i32>> = Vec::new();
            for row in board_rows {
                bingo_board.push(
                    row.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }
            boards.push(Board(bingo_board));
        }
        Ok(Game { calls, boards })
    }
}

fn check_board(board: &Vec<Vec<i32>>) -> bool {
    for i in 0..board.len() {
        let mut row = true;
        let mut col = true;
        for j in 0..board[i].len() {
            if board[i][j] != -1 {
                row = false;
            }
            if board[j][i] != -1 {
                col = false;
            }
        }
        if row || col {
            return true;
        }
    }
    false
}

fn puzzle_1(bingo: &mut Game) {
    let cloned_board = bingo.boards.clone();
    let calls = &bingo.calls;
    'outer: for call in calls {
        for board_index in 0..bingo.boards.len() {
            let board = &mut bingo.boards[board_index];
            for i in 0..board.0.len() {
                for j in 0..board.0[i].len() {
                    if board.0[i][j] == *call {
                        board.0[i][j] = -1;
                        if check_board(&board.0) {
                            let sum = {
                                let clone = &cloned_board[board_index];
                                let mut sum = 0;
                                for _i in 0..clone.0.len() {
                                    for _j in 0..clone.0[_i].len() {
                                        if board.0[_i][_j] != -1 {
                                            sum += clone.0[_i][_j];
                                        }
                                    }
                                }
                                sum
                            };
                            println!("Puzzle 1: {}", call * sum);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}

fn puzzle_2(bingo: &mut Game) {
    let cloned_board = bingo.boards.clone();
    let calls = &bingo.calls;
    let last_board = bingo.boards.len();
    let mut board_completion = 0;
    'outer: for call in calls {
        for board_index in 0..bingo.boards.len() {
            let board = &mut bingo.boards[board_index];
            for i in 0..board.0.len() {
                for j in 0..board.0[i].len() {
                    if check_board(&board.0) {
                        break;
                    }
                    if board.0[i][j] == *call {
                        board.0[i][j] = -1;
                        if check_board(&board.0) {
                            board_completion += 1;
                            if board_completion == last_board {
                                let sum = {
                                    let clone = &cloned_board[board_index];
                                    let mut sum = 0;
                                    for _i in 0..clone.0.len() {
                                        for _j in 0..clone.0[_i].len() {
                                            if board.0[_i][_j] != -1 {
                                                sum += clone.0[_i][_j];
                                            }
                                        }
                                    }
                                    sum
                                };
                                println!("Puzzle 2: {}", call * sum);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("./year/2021/inputs/day4.input").expect("Error reading file.");
    let mut game: Game = input.parse().unwrap();
    let mut new_game = game.clone();
    puzzle_1(&mut game);
    puzzle_2(&mut new_game);
}

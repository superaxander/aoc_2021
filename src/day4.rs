use std::io;

use crate::common;

fn check_row(board: &Vec<i32>, row: usize) -> bool {
    for col in 0..5 {
        if board[row * 5 + col] != -1 {
            return false;
        }
    }
    return true;
}

fn check_column(board: &Vec<i32>, col: usize) -> bool {
    for row in 0..5 {
        if board[row * 5 + col] != -1 {
            return false;
        }
    }
    return true;
}

pub fn main() -> io::Result<(i32, i32)> {
    let mut nums: Vec<i32> = Vec::new();
    let mut boards: Vec<Vec<i32>> = Vec::new();

    let lines = common::read_lines("inputs/4.txt")?;
    let mut solution_a = -1;
    let mut solution_b = 0;

    let mut first_line_parsed = false;
    let mut current_board = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if first_line_parsed {
            if line.is_empty() && current_board.len() > 0 {
                boards.push(current_board.clone());
                current_board.clear();
            } else {
                current_board.append(&mut line.split(" ").map(|x| x.trim()).filter(|x| !x.is_empty()).map(|x| x.trim().parse().unwrap()).collect());
            }
        } else {
            nums = line.split(",").map(|x| x.parse().unwrap()).collect();
            first_line_parsed = true;
        }
    }

    boards.push(current_board.clone());

    for num in nums {
        boards.retain_mut(|board|
            {
                for (i, n) in board.iter_mut().enumerate() {
                    if *n == num {
                        *n = -1;
                        if check_row(board, i / 5) || check_column(board, i % 5) {
                            if solution_a == -1 {
                                solution_a = board.iter().filter(|x| **x != -1).sum::<i32>() * num;
                            } else {
                                solution_b = board.iter().filter(|x| **x != -1).sum::<i32>() * num;
                            }
                            return false;
                        }
                        break;
                    }
                }
                return true;
            }
        );
    }

    return Ok((solution_a, solution_b));
}

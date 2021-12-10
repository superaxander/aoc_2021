use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/10.txt")?;

    let mut corruption_score = 0;
    let mut autocomplete_scores = Vec::new();

    let mut state = Vec::new();

    'line_loop: for line in lines {
        let line = line?;
        state.clear();
        for c in line.trim().chars() {
            match c {
                '[' => state.push(']'),
                '(' => state.push(')'),
                '{' => state.push('}'),
                '<' => state.push('>'),
                ']' => if let Some(s) = state.pop() {
                    if s != c {
                        corruption_score += 57;
                        continue 'line_loop;
                    }
                } else {
                    break;
                }
                ')' => if let Some(s) = state.pop() {
                    if s != c {
                        corruption_score += 3;
                        continue 'line_loop;
                    }
                } else {
                    break;
                }
                '}' => if let Some(s) = state.pop() {
                    if s != c {
                        corruption_score += 1197;
                        continue 'line_loop;
                    }
                } else {
                    break;
                }
                '>' => if let Some(s) = state.pop() {
                    if s != c {
                        corruption_score += 25137;
                        continue 'line_loop;
                    }
                } else {
                    break;
                }
                _ => panic!("Invalid character found: '{}'", c)
            }
        }

        let mut autocomplete_score = 0;
        while !state.is_empty() {
            autocomplete_score *= 5;
            match state.pop().unwrap() {
                ')' => autocomplete_score += 1,
                ']' => autocomplete_score += 2,
                '}' => autocomplete_score += 3,
                '>' => autocomplete_score += 4,
                _ => panic!("Invalid character popped")
            }
        }
        autocomplete_scores.push(autocomplete_score);
        // }
    }
    autocomplete_scores.sort_unstable();
    Ok((corruption_score, autocomplete_scores[autocomplete_scores.len() / 2]))
}

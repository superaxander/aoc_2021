use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/6.txt")?;

    let mut fishes = [0; 9];

    for line in lines {
        let line = line?;
        let split = line.trim().split(',');
        for substring in split {
            if let Ok(num) = substring.parse::<usize>() {
                fishes[num] += 1;
            }
        }
    }

    for _ in 0..80 {
        let temp = fishes[0];
        for i in 1..9 {
            fishes[i - 1] = fishes[i];
        }
        fishes[8] = temp;
        fishes[6] += temp;
    }

    let solution_a = fishes.iter().sum();

    for _ in 80..256 {
        let temp = fishes[0];
        for i in 1..9 {
            fishes[i - 1] = fishes[i];
        }
        fishes[8] = temp;
        fishes[6] += temp;
    }

    Ok((solution_a, fishes.iter().sum()))
}

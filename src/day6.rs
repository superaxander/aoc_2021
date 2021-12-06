use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
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
    
    let days = if do_b { 256 } else { 80 };

    for _ in 0..days {
        let temp = fishes[0];
        for i in 1..9 {
            fishes[i-1] = fishes[i];
        }
        fishes[8] = temp;
        fishes[6] += temp;
    }
    
    Ok(fishes.iter().sum())
}

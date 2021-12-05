use std::io;

use crate::common;

pub fn main() -> io::Result<(i32, i32)> {
    let mut sums = Vec::new();

    let lines = common::read_lines("inputs/1.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;
    let mut last_number = i32::MAX;
    for line in lines {
        if let Ok(num) = line?.trim().parse::<i32>() {
            if num > last_number {
                solution_a += 1;
                debug!("{} > {}, {}", num, last_number, solution_a)
            }
            let len = sums.len();
            if len > 2 {
                sums[len - 2] += num;
                sums[len - 1] += num;
                sums.push(num);
            } else if len > 1 {
                sums[len - 1] += num;
                sums.push(num);
            } else {
                sums.push(num)
            }

            last_number = num;
        }
    }
    let mut last_number = i32::MAX;
    for sum in sums {
        if sum > last_number {
            debug!("{} > {}", sum, last_number);
            solution_b += 1
        }
        last_number = sum;
    }

    Ok((solution_a, solution_b))
}

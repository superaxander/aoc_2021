use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut crabs = Vec::new();
    let mut sum = 0;

    for line in lines {
        let line = line?;
        let split = line.trim().split(',');
        for substring in split {
            if let Ok(num) = substring.parse::<usize>() {
                crabs.push(num);
                sum += num;
            }
        }
    }

    if do_b {
        let mean = (sum as f64 / crabs.len() as f64).round() as usize;
        let mut minimum_cost = usize::MAX;
        for m in mean - 1..mean + 1 {
            let cost = crabs
                .iter()
                .map(|x| (0..=x.abs_diff(m)).sum::<usize>())
                .sum();
            if cost < minimum_cost {
                minimum_cost = cost;
            }
        }

        Ok(minimum_cost)
    } else {
        crabs.sort_unstable();
        let median = crabs[crabs.len() / 2];
        Ok(crabs.iter().map(|x| x.abs_diff(median)).sum())
    }
}

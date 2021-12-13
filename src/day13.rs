use std::collections::HashSet;
use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/13.txt")?;

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut cnt = 0;

    let mut size_x = 0;
    let mut size_y = 0;

    let mut points_parsed = false;
    let mut is_first = true;
    for line in lines {
        let line = line?;
        let line = line.trim();
        if !points_parsed {
            if line.is_empty() {
                points_parsed = true;
            } else {
                let split: Vec<&str> = line.split(',').collect();
                let x = split[0].parse().unwrap();
                let y = split[1].parse().unwrap();
                points.insert((x, y));
            }
        } else {
            let split: Vec<&str> = line[11..].split('=').collect();
            let fold: usize = split[1].parse().unwrap();
            match split[0] {
                "x" => {
                    points = points.iter().map(|(x, y)| if *x > fold { (fold * 2 - x, *y) } else { (*x, *y) }).collect();
                    size_x = fold
                }
                "y" => {
                    points = points.iter().map(|(x, y)| if *y > fold { (*x, fold * 2 - y) } else { (*x, *y) }).collect();
                    size_y = fold
                },
                _ => debug!("Invalid axis: {}", split[0])
            }
            if is_first {
                cnt = points.len();
                is_first = false;
            }
        }
    }

    let mut string = String::new();
    for y in 0..size_y {
        for x in 0..size_x {
            if points.contains(&(x, y)) {
                string.push('#')
            } else {
                string.push('-');
            }
        }
        string.push('\n')
    }
    print!("{}", string);

    Ok((cnt, 0))
}

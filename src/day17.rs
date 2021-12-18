use std::io;

use crate::common;

pub fn main() -> io::Result<(i64, i64)> {
    let lines = common::read_lines("inputs/17.txt")?;

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        let x_y_split: Vec<&str> = line.split(", y=").collect();
        let x_split: Vec<&str> = x_y_split[0][15..].split("..").collect();
        let y_split: Vec<&str> = x_y_split[1].split("..").collect();
        min_x = x_split[0].parse().unwrap();
        max_x = x_split[1].parse().unwrap();
        min_y = y_split[0].parse().unwrap();
        max_y = y_split[1].parse().unwrap();
    }

    // Find x velocity
    let min_x_velocity = (2f64*min_x as f64).sqrt() as i64;

    let mut solution_b = 0;

    for x_velocity in min_x_velocity..=max_x {
        for y_velocity in min_y..-min_y {
            let mut x = 0;
            let mut y = 0;
            let mut dx = x_velocity;
            let mut dy = y_velocity;
            while x <= max_x && y >= min_y {
                x += dx;
                y += dy;
                dx = 0.max(dx - 1);
                dy -= 1;
                if x >= min_x && y <= max_y && x <= max_x && y >= min_y {
                    solution_b += 1;
                    break;
                }
            }
        }
    }

    Ok(((min_y * (min_y + 1)) / 2, solution_b))
}

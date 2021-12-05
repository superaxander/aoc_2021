use std::io;

use crate::common;

pub fn main() -> io::Result<(i32, i32)> {
    let lines = common::read_lines("inputs/2.txt")?;

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut depth_adjusted = 0;

    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.trim().split(" ").collect();
        if let Ok(x) = split[1].parse::<i32>() {
            match split[0] {
                "forward" => {
                    horizontal_position += x;
                    depth_adjusted += x * aim;
                }
                "up" => {
                    depth -= x;
                    aim -= x;
                }
                "down" => {
                    depth += x;
                    aim += x;
                }
                _ => error!("Invalid input"),
            }
        }
    }

    return Ok((
        horizontal_position * depth,
        horizontal_position * depth_adjusted,
    ));
}

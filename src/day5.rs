use std::cmp::max;
use std::io;

use crate::common;

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn add_points(&self, map: &mut Vec<usize>, size_x: i32) {
        let delta_x = self.x1 - self.x2;
        let delta_y = self.y1 - self.y2;
        #[allow(clippy::comparison_chain)]
        if delta_x > 0 {
            let a = delta_y / delta_x;
            let b = 1 + size_x * a;
            let c = (self.y1 - a * self.x1) * size_x;
            for x in self.x2..=self.x1 {
                map[(b * x + c) as usize] += 1;
            }
        } else if delta_x == 0 {
            if delta_y > 0 {
                for y in self.y2..=self.y1 {
                    map[(self.x1 + (y * size_x)) as usize] += 1;
                }
            } else {
                for y in self.y1..=self.y2 {
                    map[(self.x1 + (y * size_x)) as usize] += 1;
                }
            }
        } else {
            let a = delta_y / delta_x;
            let b = 1 + size_x * a;
            let c = (self.y1 - a * self.x1) * size_x;
            for x in self.x1..=self.x2 {
                map[(b * x + c) as usize] += 1;
            }
        }
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut vents = Vec::new();

    let lines = common::read_lines("inputs/5.txt")?;

    let mut size_x = 0;
    let mut size_y = 0;

    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.trim().split(" -> ").collect();
        let line_start: Vec<&str> = split[0].split(',').collect();
        let line_end: Vec<&str> = split[1].split(',').collect();
        let vent = Line {
            x1: line_start[0].parse().unwrap(),
            y1: line_start[1].parse().unwrap(),
            x2: line_end[0].parse().unwrap(),
            y2: line_end[1].parse().unwrap(),
        };
        size_x = max(size_x, max(vent.x1, vent.x2));
        size_y = max(size_y, max(vent.y1, vent.y2));
        vents.push(vent)
    }

    let mut map = vec![0; (size_x as usize + 1) * (size_y as usize + 1)];
    if !do_b {
        vents
            .iter()
            .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
            .for_each(|v| v.add_points(&mut map, size_x));
    } else {
        vents.iter().for_each(|v| v.add_points(&mut map, size_x));
    }

    return Ok(map.iter().filter(|x| **x > 1).count());
}

use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/11.txt")?;

    let mut map = Vec::new();
    let mut size_x: i32 = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        size_x = line.len() as i32;
        for c in line.chars() {
            map.push(c as usize - '0' as usize);
        }
    }

    let size_y = map.len() as i32 / size_x;

    fn increment(map: &mut Vec<usize>, x: i32, y: i32, size_x: i32, size_y: i32) {
        let idx = (x + y * size_x) as usize; 
        if x >= 0 && x < size_x && y >= 0 && y < size_y && map[idx] < 10 {
            map[idx] += 1;
        }
    }

    let mut count = 0;

    for time in 0..usize::MAX {
        map.iter_mut().for_each(|energy| *energy += 1);

        let mut flashed = map.iter().enumerate().filter_map(|(i, energy)| if *energy == 10 { Some(i) } else { None }).collect::<Vec<usize>>();
        while !flashed.is_empty() {
            if time < 100 {
                count += flashed.len();
            }
            for i in &flashed {
                let x = *i as i32 % size_x;
                let y = *i as i32 / size_x;
                
                increment(&mut map, x - 1, y - 1, size_x, size_y);
                increment(&mut map, x, y - 1, size_x, size_y);
                increment(&mut map, x + 1, y - 1, size_x, size_y);
                increment(&mut map, x - 1, y, size_x, size_y);
                map[*i] += 1;
                increment(&mut map, x + 1, y, size_x, size_y);
                increment(&mut map, x - 1, y + 1, size_x, size_y);
                increment(&mut map, x, y + 1, size_x, size_y);
                increment(&mut map, x + 1, y + 1, size_x, size_y);
            }
            flashed = map.iter().enumerate().filter_map(|(i, energy)| if *energy == 10 { Some(i) } else { None }).collect::<Vec<usize>>();
        }

        let mut amount = 0;
        map.iter_mut().filter(|energy| **energy > 9).for_each(|energy| {
            *energy = 0;
            amount += 1
        });
        if amount == map.len() {
            return Ok((count, time))
        }
    }

    Ok((count, 0))
}

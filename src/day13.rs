use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/13.txt")?;

    let mut temp: Vec<usize> = Vec::new();
    let mut folds: Vec<(bool, usize)> = Vec::new();

    let mut size_x = 0;
    let mut size_y = 0;

    let mut points_parsed = false;
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
                size_x = size_x.max(x);
                size_y = size_y.max(y);
                temp.push(x);
                temp.push(y);
            }
        } else {
            let split: Vec<&str> = line[11..].split('=').collect();
            match split[0] {
                "x" => folds.push((false, split[1].parse().unwrap())),
                "y" => folds.push((true, split[1].parse().unwrap())),
                _ => debug!("Invalid axis: {}", split[0])
            }
        }
    }
    size_x += 1;
    size_y += 1;

    let mut map = vec![false; size_x * size_y];

    for i in (0..temp.len()).step_by(2) {
        map[temp[i] + temp[i + 1] * size_x] = true;
    }

    let mut adjusted_size_x = size_x;
    let mut cnt = 0;
    let mut is_first = true;
    for (is_y, fold) in folds {
        if is_y {
            for y in 0..fold {
                for x in 0..adjusted_size_x {
                    map[x + y * size_x] |= map[x + (size_y - y - 1) * size_x];
                }
            }
            size_y = fold
        } else {
            for y in 0..size_y {
                for x in 0..fold {
                    map[x + y * size_x] |= map[adjusted_size_x - x - 1 + y * size_x];
                }
            }
            adjusted_size_x = fold
        }

        if is_first {
            for y in 0..size_y {
                for x in 0..adjusted_size_x {
                    if map[x + y * size_x] {
                        cnt += 1;
                    }
                }
            }
            is_first = false;
        }
    }
    let mut string = String::new();
    for y in 0..size_y {
        for x in 0..adjusted_size_x {
            if map[x + y * size_x] {
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

use std::collections::HashSet;
use std::io;

use crate::common;

const BIT_SIZE: usize = usize::BITS as usize;

macro_rules! get_bit {
    ($bits:expr, $idx:expr) => {{
        ($bits[$idx / BIT_SIZE] >> (BIT_SIZE - 1 - $idx % BIT_SIZE)) & 1
    }};
}

pub fn main() -> io::Result<(usize, usize)> {
    let mut lines = common::read_lines("inputs/20.txt")?;

    let mut idx = 0;
    let mut algorithm: Vec<usize> = Vec::new();

    for c in lines.next().unwrap()?.trim().chars() {
        match c {
            '.' => {
                if idx % BIT_SIZE == 0 {
                    algorithm.push(0);
                } else {
                    let l = algorithm.len();
                    algorithm[l - 1] <<= 1;
                }
                idx += 1;
            }
            '#' => {
                if idx % BIT_SIZE == 0 {
                    algorithm.push(1);
                } else {
                    let l = algorithm.len();
                    algorithm[l - 1] = (algorithm[l - 1] << 1) | 1
                }
                idx += 1;
            }

            _ => panic!("Invalid character: {}", c),
        }
    }

    let mut map = HashSet::new();

    let mut size_x: i64 = 0;
    let mut y: i64 = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        size_x = line.len() as i64;

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y));
            }
        }
        y += 1;
    }

    let mut min_x = -1;
    let mut min_y = -1;

    size_x += 1;
    let mut size_y = y + 1;

    let do_flip = get_bit!(algorithm, 0) > 0 && get_bit!(algorithm, 511) == 0;
    let mut flip = false;

    let mut solution_a = 0;

    for iteration in 0..50 {
        let mut copy = map.clone();

        for i in min_x..=size_x {
            for j in min_y..=size_y {
                let mut num = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        num <<= 1;
                        if map.contains(&(i + l, j + k))
                            || (flip
                                && do_flip
                                && (i + l <= min_x
                                    || j + k <= min_y
                                    || i + l >= size_x
                                    || j + k >= size_y))
                        {
                            num |= 1;
                        }
                    }
                }

                if get_bit!(algorithm, num) > 0 {
                    copy.insert((i, j));
                } else {
                    copy.remove(&(i, j));
                }
            }
        }

        min_x -= 1;
        min_y -= 1;
        size_x += 1;
        size_y += 1;

        map = copy;
        flip = !flip;

        if iteration == 1 {
            solution_a = map.len();
        }
    }

    Ok((solution_a, map.len()))
}

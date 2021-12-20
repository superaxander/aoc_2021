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

    let mut map = Vec::new();

    let mut size_x = 0;
    let mut size_y = 100;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        size_x = line.len() + 100;

        if size_y == 100 {
            map.extend(vec![false; size_x * 50])
        }

        map.extend([false; 50]);

        for c in line.chars() {
            map.push(c == '#');
        }

        map.extend([false; 50]);
        size_y += 1;
    }

    map.extend(vec![false; size_x * 50]);

    let do_flip = get_bit!(algorithm, 0) > 0 && get_bit!(algorithm, 511) == 0;
    let mut flip = false;

    let mut solution_a = 0;

    let size_x = size_x as i64;
    let size_y = size_y as i64;

    let mut bound_min_x = 49;
    let mut bound_max_x = size_x - 49;

    let mut bound_min_y = 49;
    let mut bound_max_y = size_y - 49;

    let mut copy = map.clone();

    for iteration in 0..50 {
        for i in bound_min_x..bound_max_x {
            for j in bound_min_y..bound_max_y {
                let mut num = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        num <<= 1;

                        if i + l <= bound_min_x
                            || j + k <= bound_min_y
                            || i + l >= bound_max_x - 1
                            || j + k >= bound_max_y - 1
                        {
                            if flip {
                                num |= 1;
                            }
                        } else if map[(i + l + (j + k) * size_x) as usize] {
                            num |= 1;
                        }
                    }
                }

                if get_bit!(algorithm, num) > 0 {
                    copy[(i + j * size_x) as usize] = true;
                } else {
                    copy[(i + j * size_x) as usize] = false;
                }
            }
        }

        bound_min_x -= 1;
        bound_min_y -= 1;
        bound_max_x += 1;
        bound_max_y += 1;

        map.copy_from_slice(&copy);
        flip ^= do_flip;

        if iteration == 1 {
            solution_a = map.iter().filter(|b| **b).count();
        }
    }

    Ok((solution_a, map.iter().filter(|b| **b).count()))
}

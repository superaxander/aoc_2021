use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/25.txt")?;

    let mut map = Vec::new();
    let mut size_x = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        size_x = line.len();

        map.extend(line.chars());
    }

    let size_y = map.len() / size_x;

    let mut copy = map.clone();

    for i in 0..usize::MAX {
        let mut changed = false;
        for j in 0..2 {
            for y in 0..size_y {
                for x in 0..size_x {
                    let idx = y * size_x + x;
                    match map[idx] {
                        '>' => {
                            if j != 0 {
                                continue;
                            }
                            let right = y * size_x + (x + 1) % size_x;

                            if map[right] == '.' {
                                copy[idx] = '.';
                                copy[right] = '>';
                            }
                        }
                        'v' => {
                            if j == 0 {
                                continue;
                            }
                            let down = ((y + 1) % size_y) * size_x + x;

                            if map[down] == '.' {
                                copy[idx] = '.';
                                copy[down] = 'v';
                            }
                        }
                        _ => {}
                    }
                }
            }
            if map != copy {
                changed = true;
                map.copy_from_slice(&copy);
            }
        }

        if !changed {
            return Ok((i + 1, 0));
        }

        // mem::swap()
    }

    Ok((0, 0))
}

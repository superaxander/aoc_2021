use std::io;

use crate::common;

fn get_neighbours(x: usize, y: usize, size_x: usize, size_y: usize) -> Vec<usize> {
    let mut neighbours = Vec::new();
    if (y as i32 - 1) >= 0 {
        neighbours.push((y - 1) * size_x + x);
    }
    if (x as i32 - 1) >= 0 {
        neighbours.push(y * size_x + x - 1);
    }
    if (y as i32 + 1) < size_y as i32 {
        neighbours.push((y + 1) * size_x + x);
    }
    if (x as i32 + 1) < size_x as i32 {
        neighbours.push(y * size_x + x + 1);
    }

    neighbours
}

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/9.txt")?;

    let mut map = Vec::new();
    let mut size_x = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        size_x = line.len();
        for n in line.chars().map(|c| c as usize - '0' as usize) {
            map.push(n)
        }
    }

    let size_y = map.len() / size_x;

    let mut sum = 0;
    let mut low_points = Vec::new();

    for y in 0..map.len() / size_x {
        'x_loop: for x in 0..size_x {
            let value = map[y * size_x + x];
            for neighbour in get_neighbours(x, y, size_x, size_y) {
                if map[neighbour] <= value {
                    continue 'x_loop;
                }
            }

            sum += map[y * size_x + x] + 1;
            low_points.push((x, y));
        }
    }
    let mut basins = Vec::new();

    for (x, y) in low_points {
        let mut checked = vec![x + y * size_x];
        let mut to_check_next = get_neighbours(x, y, size_x, size_y);
        let mut neighbours = Vec::new();
        let mut sum = 1;

        while !to_check_next.is_empty() {
            neighbours.extend(&to_check_next);
            to_check_next.clear();
            for neighbour in &neighbours {
                if !checked.contains(neighbour) {
                    let neighbours_neighbours =
                        get_neighbours(neighbour % size_x, neighbour / size_x, size_x, size_y);

                    let val = map[*neighbour];

                    if val == 9 {
                        continue;
                    }

                    sum += 1;
                    checked.push(*neighbour);
                    to_check_next.extend(&neighbours_neighbours);
                }
            }
            neighbours.clear();
        }

        basins.push(sum);
    }

    basins.sort_unstable_by_key(|n| -(*n as i32));
    Ok((sum, basins[0..3].iter().product()))
}

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io;

use crate::common;

type Coordinate = (i64, i64, i64);

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/19.txt")?;

    let mut scanners: Vec<HashSet<Coordinate>> = Vec::new();

    let mut centers: Vec<Vec<Coordinate>> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            scanners.push(HashSet::new());
            centers.push(vec![(0, 0, 0)]);
            continue;
        }

        let mut coords_iter = line.split(',').map(|s| s.parse::<i64>().unwrap());
        scanners.last_mut().unwrap().insert((
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
        ));
    }

    while scanners.len() > 1 {
        'outer: for i in 0..scanners.len() {
            for j in i + 1..scanners.len() {
                // Loop through possible scanner positions
                let mut possible_positions = HashMap::new();
                for a in &scanners[i] {
                    for b in &scanners[j] {
                        for p in shifts(b, a) {
                            let (d, c) = p;
                            assert_eq!(
                                shift_back(b, &c, &d),
                                *a,
                                "Testing {:?}, {:?}, with {:?}",
                                c,
                                b,
                                d
                            );
                            match possible_positions.entry(p) {
                                Entry::Occupied(mut e) => {
                                    *(e.get_mut()) += 1;
                                }
                                Entry::Vacant(e) => {
                                    e.insert(1);
                                }
                            }
                        }
                    }
                }
                let possible_positions = possible_positions
                    .iter()
                    .filter(|(_, i)| **i >= 12)
                    .collect::<Vec<(&((usize, usize), Coordinate), &usize)>>();
                assert!(possible_positions.len() <= 1);
                if let Some(((flip_type, p), _)) = possible_positions.first() {
                    let mut scanner = scanners[i].clone();

                    for beacon in &scanners[j] {
                        let shifted_beacon = shift_back(beacon, p, flip_type);
                        scanner.insert(shifted_beacon);
                    }
                    scanners.remove(j);
                    scanners.remove(i);
                    scanners.push(scanner);
                    let j_centers = centers.remove(j);
                    let mut i_centers = centers.remove(i);
                    i_centers.extend(j_centers.iter().map(|c| shift_back(c, p, flip_type)));
                    centers.push(i_centers);
                    break 'outer;
                }
            }
        }
    }

    let mut max = 0;

    for i in 0..centers[0].len() {
        for j in i + 1..centers[0].len() {
            let val = manhattan(centers[0][i], centers[0][j]);
            if val > max {
                max = val;
            }
        }
    }

    Ok((scanners[0].len(), max))
}

fn manhattan(b1: Coordinate, b2: Coordinate) -> usize {
    (b1.0.abs_diff(b2.0) + b1.1.abs_diff(b2.1) + b1.2.abs_diff(b2.2)) as usize
}

const POSSIBLE_FLIPS: [(bool, bool, bool); 7] = [
    (false, false, false),
    (false, false, true),
    (false, true, false),
    (true, false, false),
    (true, false, true),
    (true, true, false),
    (true, true, true),
];

fn shift(b1: &Coordinate, b2: &Coordinate, flip_type: &(usize, usize)) -> Coordinate {
    let flips = POSSIBLE_FLIPS[flip_type.0];
    match flip_type.1 {
        0 => (
            if flips.0 { b2.0 + b1.0 } else { b2.0 - b1.0 },
            if flips.1 { b2.1 + b1.1 } else { b2.1 - b1.1 },
            if flips.2 { b2.2 + b1.2 } else { b2.2 - b1.2 },
        ),
        1 => (
            if flips.0 { b2.0 + b1.0 } else { b2.0 - b1.0 },
            if flips.1 { b2.2 + b1.1 } else { b2.2 - b1.1 },
            if flips.2 { b2.1 + b1.2 } else { b2.1 - b1.2 },
        ),
        2 => (
            if flips.0 { b2.1 + b1.0 } else { b2.1 - b1.0 },
            if flips.1 { b2.0 + b1.1 } else { b2.0 - b1.1 },
            if flips.2 { b2.2 + b1.2 } else { b2.2 - b1.2 },
        ),
        3 => (
            if flips.0 { b2.1 + b1.0 } else { b2.1 - b1.0 },
            if flips.1 { b2.2 + b1.1 } else { b2.2 - b1.1 },
            if flips.2 { b2.0 + b1.2 } else { b2.0 - b1.2 },
        ),
        4 => (
            if flips.0 { b2.2 + b1.0 } else { b2.2 - b1.0 },
            if flips.1 { b2.1 + b1.1 } else { b2.1 - b1.1 },
            if flips.2 { b2.0 + b1.2 } else { b2.0 - b1.2 },
        ),
        5 => (
            if flips.0 { b2.2 + b1.0 } else { b2.2 - b1.0 },
            if flips.1 { b2.0 + b1.1 } else { b2.0 - b1.1 },
            if flips.2 { b2.1 + b1.2 } else { b2.1 - b1.2 },
        ),
        _ => panic!("Wrong value"),
    }
}

fn shift_back(b1: &Coordinate, b2: &Coordinate, flip_type: &(usize, usize)) -> Coordinate {
    let flips = POSSIBLE_FLIPS[flip_type.0];
    match flip_type.1 {
        0 => (
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
            if flips.1 { b2.1 - b1.1 } else { b1.1 + b2.1 },
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
        ),
        1 => (
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
            if flips.1 { b2.1 - b1.1 } else { b2.1 + b1.1 },
        ),
        2 => (
            if flips.1 { b2.1 - b1.1 } else { b2.1 + b1.1 },
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
        ),
        3 => (
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
            if flips.1 { b2.1 - b1.1 } else { b2.1 + b1.1 },
        ),
        4 => (
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
            if flips.1 { b2.1 - b1.1 } else { b1.1 + b2.1 },
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
        ),
        5 => (
            if flips.1 { b2.1 - b1.1 } else { b2.1 + b1.1 },
            if flips.2 { b2.2 - b1.2 } else { b2.2 + b1.2 },
            if flips.0 { b2.0 - b1.0 } else { b2.0 + b1.0 },
        ),
        _ => panic!("Wrong value"),
    }
}

fn shifts(b1: &Coordinate, b2: &Coordinate) -> Vec<((usize, usize), Coordinate)> {
    let mut shifts = Vec::new();
    for i in 0..POSSIBLE_FLIPS.len() {
        shifts.push(((i, 0), shift(b1, b2, &(i, 0))));
        shifts.push(((i, 1), shift(b1, b2, &(i, 1))));
        shifts.push(((i, 2), shift(b1, b2, &(i, 2))));
        shifts.push(((i, 3), shift(b1, b2, &(i, 3))));
        shifts.push(((i, 4), shift(b1, b2, &(i, 4))));
        shifts.push(((i, 5), shift(b1, b2, &(i, 5))));
    }

    shifts
}

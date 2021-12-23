use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::io;

use crate::common;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, Eq, PartialEq)]
struct Node<const SIZE: usize> {
    state: [usize; SIZE],
    gscore: usize,
}

impl<const SIZE: usize> Node<SIZE> {
    fn new(state: [usize; SIZE], gscore: usize) -> Self {
        Node { state, gscore }
    }
}

impl<const SIZE: usize> PartialOrd for Node<SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.gscore.partial_cmp(&self.gscore)
    }

    fn lt(&self, other: &Self) -> bool {
        self.gscore >= other.gscore
    }

    fn le(&self, other: &Self) -> bool {
        self.gscore > other.gscore
    }

    fn gt(&self, other: &Self) -> bool {
        self.gscore <= other.gscore
    }

    fn ge(&self, other: &Self) -> bool {
        self.gscore < other.gscore
    }
}

impl<const SIZE: usize> Ord for Node<SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gscore.cmp(&other.gscore)
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let lines = common::read_lines("inputs/23.txt")?;

    if !do_b {
        let mut starting_state: [usize; 15] = [0; 15];
        fn to_state_coord(x: usize, y: usize) -> usize {
            if y == 1 {
                if x < 3 {
                    x - 1
                } else if x < 5 {
                    x - 2
                } else if x < 7 {
                    x - 3
                } else if x < 9 {
                    x - 4
                } else {
                    x - 6
                }
            } else if x == 3 {
                7 + (y - 2)
            } else if x == 5 {
                9 + (y - 2)
            } else if x == 7 {
                11 + (y - 2)
            } else if x == 9 {
                13 + (y - 2)
            } else {
                panic!("Wrong coordinates: {} {}", x, y);
            }
        }

        for (y, line) in lines.enumerate() {
            let line = line?;

            for (x, c) in line.chars().enumerate() {
                match c {
                    'A' => starting_state[to_state_coord(x, y)] = 1,
                    'B' => starting_state[to_state_coord(x, y)] = 10,
                    'C' => starting_state[to_state_coord(x, y)] = 100,
                    'D' => starting_state[to_state_coord(x, y)] = 1000,
                    _ => {}
                }
            }
        }

        Ok(find_minimum(
            starting_state,
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 10, 10, 100, 100, 1000, 1000],
        ))
    } else {
        let mut starting_state: [usize; 23] = [
            0, 0, 0, 0, 0, 0, 0, 0, 1000, 1000, 0, 0, 100, 10, 0, 0, 10, 1, 0, 0, 1, 100, 0,
        ];

        fn to_state_coord(x: usize, y: usize) -> usize {
            if y == 1 {
                if x < 3 {
                    x - 1
                } else if x < 5 {
                    x - 2
                } else if x < 7 {
                    x - 3
                } else if x < 9 {
                    x - 4
                } else {
                    x - 6
                }
            } else if x == 3 {
                7 + (y - 2) * 3
            } else if x == 5 {
                11 + (y - 2) * 3
            } else if x == 7 {
                15 + (y - 2) * 3
            } else if x == 9 {
                19 + (y - 2) * 3
            } else {
                panic!("Wrong coordinates: {} {}", x, y);
            }
        }

        for (y, line) in lines.enumerate() {
            let line = line?;

            for (x, c) in line.chars().enumerate() {
                match c {
                    'A' => starting_state[to_state_coord(x, y)] = 1,
                    'B' => starting_state[to_state_coord(x, y)] = 10,
                    'C' => starting_state[to_state_coord(x, y)] = 100,
                    'D' => starting_state[to_state_coord(x, y)] = 1000,
                    _ => {}
                }
            }
        }

        Ok(find_minimum(
            starting_state,
            [
                0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 10, 10, 10, 10, 100, 100, 100, 100, 1000, 1000,
                1000, 1000,
            ],
        ))
    }
}

fn find_minimum<const SIZE: usize>(
    starting_state: [usize; SIZE],
    final_state: [usize; SIZE],
) -> usize {
    let mut gscores: HashMap<u64, usize> = HashMap::new();
    gscores.insert(calculate_hash(&starting_state), 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(Node::new(starting_state, 0));

    while !frontier.is_empty() {
        let Node {
            state: current_state,
            gscore: current_score,
        } = frontier.pop().unwrap();

        if current_state == final_state {
            return current_score;
        }

        let current_hash = calculate_hash(&current_state);

        'outer: for i in 0..7 {
            match current_state[i] {
                1 => {
                    if path_blocked(&current_state, i, 2) {
                        continue 'outer;
                    }

                    let mut lowest_0 = 6;
                    for (j, item) in current_state
                        .iter()
                        .enumerate()
                        .take(7 + (SIZE - 7) / 4)
                        .skip(7)
                    {
                        if *item == 0 {
                            lowest_0 = j;
                        } else if *item != 1 {
                            lowest_0 = 6;
                            break;
                        }
                    }

                    if lowest_0 > 6 {
                        let cloned = swap(&current_state, i, lowest_0);
                        let tentative_gscore = gscores[&current_hash]
                            + current_state[i] * distance_hallway_room::<SIZE>(i, lowest_0);
                        update_score(&mut gscores, &mut frontier, &cloned, tentative_gscore)
                    }
                }
                10 => {
                    if path_blocked(&current_state, i, 3) {
                        continue 'outer;
                    }

                    let mut lowest_0 = 6 + (SIZE - 7) / 4;
                    for (j, item) in current_state
                        .iter()
                        .enumerate()
                        .take(7 + 2 * (SIZE - 7) / 4)
                        .skip(7 + (SIZE - 7) / 4)
                    {
                        if *item == 0 {
                            lowest_0 = j;
                        } else if *item != 10 {
                            lowest_0 = 6 + (SIZE - 7) / 4;
                            break;
                        }
                    }

                    if lowest_0 > 6 + (SIZE - 7) / 4 {
                        let cloned = swap(&current_state, i, lowest_0);
                        let tentative_gscore = gscores[&current_hash]
                            + current_state[i] * distance_hallway_room::<SIZE>(i, lowest_0);
                        update_score(&mut gscores, &mut frontier, &cloned, tentative_gscore)
                    }
                }
                100 => {
                    if path_blocked(&current_state, i, 4) {
                        continue 'outer;
                    }

                    let mut lowest_0 = 6 + 2 * (SIZE - 7) / 4;
                    for (j, item) in current_state
                        .iter()
                        .enumerate()
                        .take(7 + 3 * (SIZE - 7) / 4)
                        .skip(7 + 2 * (SIZE - 7) / 4)
                    {
                        if *item == 0 {
                            lowest_0 = j;
                        } else if *item != 100 {
                            lowest_0 = 6 + 2 * (SIZE - 7) / 4;
                            break;
                        }
                    }

                    if lowest_0 > 6 + 2 * (SIZE - 7) / 4 {
                        let cloned = swap(&current_state, i, lowest_0);
                        let tentative_gscore = gscores[&current_hash]
                            + current_state[i] * distance_hallway_room::<SIZE>(i, lowest_0);
                        update_score(&mut gscores, &mut frontier, &cloned, tentative_gscore)
                    }
                }
                1000 => {
                    if path_blocked(&current_state, i, 5) {
                        continue 'outer;
                    }

                    let mut lowest_0 = 6 + 3 * (SIZE - 7) / 4;
                    for (j, item) in current_state
                        .iter()
                        .enumerate()
                        .skip(7 + 3 * (SIZE - 7) / 4)
                    {
                        if *item == 0 {
                            lowest_0 = j;
                        } else if *item != 1000 {
                            lowest_0 = 6 + 3 * (SIZE - 7) / 4;
                            break;
                        }
                    }

                    if lowest_0 > 6 + 3 * (SIZE - 7) / 4 {
                        let cloned = swap(&current_state, i, lowest_0);
                        let tentative_gscore = gscores[&current_hash]
                            + current_state[i] * distance_hallway_room::<SIZE>(i, lowest_0);
                        update_score(&mut gscores, &mut frontier, &cloned, tentative_gscore)
                    }
                }
                _ => {}
            }
        }

        // Check if the top amphipods in each room can move to a position in the hallway
        // Need to make sure that nodes in correct stacks can't move so if you're in the right
        // room and everyone behind you is also in the correct room you can't move.

        check_move_to_hallway_stack(
            &mut gscores,
            &mut frontier,
            &current_state,
            current_hash,
            0,
            1,
        );
        check_move_to_hallway_stack(
            &mut gscores,
            &mut frontier,
            &current_state,
            current_hash,
            1,
            10,
        );
        check_move_to_hallway_stack(
            &mut gscores,
            &mut frontier,
            &current_state,
            current_hash,
            2,
            100,
        );
        check_move_to_hallway_stack(
            &mut gscores,
            &mut frontier,
            &current_state,
            current_hash,
            3,
            1000,
        );
    }

    0
}

fn check_move_to_hallway_stack<const SIZE: usize>(
    gscores: &mut HashMap<u64, usize>,
    frontier: &mut BinaryHeap<Node<SIZE>>,
    current_state: &[usize; SIZE],
    current_hash: u64,
    room: usize,
    room_type: usize,
) {
    let mut in_wrong_stack = false;
    let mut highest_non_zero = 7 + (room + 1) * ((SIZE - 7) / 4);
    for i in ((7 + room * ((SIZE - 7) / 4))..(7 + (room + 1) * ((SIZE - 7) / 4))).rev() {
        match current_state[i] {
            0 => {
                break;
            }
            it => {
                if it != room_type {
                    in_wrong_stack = true;
                }
                highest_non_zero = i;
            }
        }
    }
    if highest_non_zero < 23 && in_wrong_stack {
        check_move_to_hallway(
            gscores,
            frontier,
            current_state,
            highest_non_zero,
            current_hash,
            &mut (0..room + 2).rev(),
        );
        check_move_to_hallway(
            gscores,
            frontier,
            current_state,
            highest_non_zero,
            current_hash,
            &mut (room + 2..7),
        );
    }
}

fn check_move_to_hallway<const SIZE: usize>(
    gscores: &mut HashMap<u64, usize>,
    frontier: &mut BinaryHeap<Node<SIZE>>,
    current_state: &[usize; SIZE],
    highest_non_zero: usize,
    current_hash: u64,
    iter: &mut dyn Iterator<Item = usize>,
) {
    for i in iter {
        if current_state[i] == 0 {
            let cloned = swap(current_state, highest_non_zero, i);
            let tentative_gscore = gscores[&current_hash]
                + current_state[highest_non_zero]
                    * distance_hallway_room::<SIZE>(i, highest_non_zero);
            update_score(gscores, frontier, &cloned, tentative_gscore)
        } else {
            break;
        }
    }
}

fn path_blocked<const SIZE: usize>(current_state: &[usize; SIZE], from: usize, to: usize) -> bool {
    for item in current_state
        .iter()
        .take(to.max(from))
        .skip(to.min(from + 1))
    {
        if *item > 0 {
            return true;
        }
    }
    false
}

fn swap<const SIZE: usize>(current_state: &[usize; SIZE], from: usize, to: usize) -> [usize; SIZE] {
    let mut cloned = *current_state;
    cloned[from] = 0;
    cloned[to] = current_state[from];
    cloned
}

fn update_score<const SIZE: usize>(
    gscores: &mut HashMap<u64, usize>,
    frontier: &mut BinaryHeap<Node<SIZE>>,
    cloned: &[usize; SIZE],
    tentative_gscore: usize,
) {
    let hash = calculate_hash(&cloned);
    if tentative_gscore < *gscores.get(&hash).unwrap_or(&usize::MAX) {
        let new_state = Node::new(*cloned, tentative_gscore);
        gscores.insert(hash, tentative_gscore);
        frontier.push(new_state);
    }
}

#[cfg(test)]
mod tests {
    use crate::day23::distance_hallway_room;

    #[test]
    fn test_distance_hallway_room() {
        const DISTANCES: [(usize, usize, usize); 108] = [
            (0, 7, 3),
            (0, 8, 4),
            (0, 9, 5),
            (0, 10, 6),
            (1, 7, 2),
            (1, 8, 3),
            (1, 9, 4),
            (1, 10, 5),
            (2, 7, 2),
            (2, 8, 3),
            (2, 9, 4),
            (2, 10, 5),
            (3, 7, 4),
            (3, 8, 5),
            (3, 9, 6),
            (3, 10, 7),
            (4, 7, 6),
            (4, 8, 7),
            (4, 9, 8),
            (4, 10, 9),
            (5, 7, 8),
            (5, 8, 9),
            (5, 9, 10),
            (5, 10, 11),
            (6, 7, 9),
            (6, 8, 10),
            (6, 9, 11),
            (6, 10, 12),
            (0, 11, 5),
            (0, 12, 6),
            (0, 13, 7),
            (0, 14, 8),
            (1, 11, 4),
            (1, 12, 5),
            (1, 13, 6),
            (1, 14, 7),
            (2, 11, 2),
            (2, 12, 3),
            (2, 13, 4),
            (2, 14, 5),
            (3, 11, 2),
            (3, 12, 3),
            (3, 13, 4),
            (3, 14, 5),
            (4, 11, 4),
            (4, 12, 5),
            (4, 13, 6),
            (4, 14, 7),
            (5, 11, 6),
            (5, 12, 7),
            (5, 13, 8),
            (5, 14, 9),
            (6, 11, 7),
            (6, 12, 8),
            (6, 13, 9),
            (6, 14, 10),
            (0, 15, 7),
            (0, 16, 8),
            (0, 17, 9),
            (0, 18, 10),
            (1, 15, 6),
            (1, 16, 7),
            (1, 17, 8),
            (1, 18, 9),
            (2, 15, 4),
            (2, 16, 5),
            (2, 17, 6),
            (2, 18, 7),
            (3, 15, 2),
            (3, 16, 3),
            (3, 17, 4),
            (3, 18, 5),
            (4, 15, 2),
            (4, 16, 3),
            (4, 17, 4),
            (4, 18, 5),
            (5, 15, 4),
            (5, 16, 5),
            (5, 17, 6),
            (5, 18, 7),
            (6, 15, 5),
            (6, 16, 6),
            (6, 17, 7),
            (6, 18, 8),
            (1, 19, 8),
            (1, 20, 9),
            (1, 21, 10),
            (1, 22, 11),
            (2, 19, 6),
            (2, 20, 7),
            (2, 21, 8),
            (2, 22, 9),
            (3, 19, 4),
            (3, 20, 5),
            (3, 21, 6),
            (3, 22, 7),
            (4, 19, 2),
            (4, 20, 3),
            (4, 21, 4),
            (4, 22, 5),
            (5, 19, 2),
            (5, 20, 3),
            (5, 21, 4),
            (5, 22, 5),
            (6, 19, 3),
            (6, 20, 4),
            (6, 21, 5),
            (6, 22, 6),
        ];

        for (hallway_pos, room_pos, distance) in DISTANCES {
            assert_eq!(
                distance,
                distance_hallway_room(hallway_pos, room_pos),
                "Testing with {} {}",
                hallway_pos,
                room_pos
            );
        }
    }
}

fn distance_hallway_room<const SIZE: usize>(hallway_pos: usize, room_pos: usize) -> usize {
    let room = (room_pos - 7) / ((SIZE - 7) / 4);
    let room_height = (room_pos - 7) % ((SIZE - 7) / 4);
    match hallway_pos {
        0 => 3 + room * 2 + room_height,
        1 => 2 + room * 2 + room_height,
        2 => {
            if room > 0 {
                2 + (room - 1) * 2 + room_height
            } else {
                2 + room_height
            }
        }
        3 => {
            if room > 1 {
                2 + (room - 2) * 2 + room_height
            } else {
                2 + (1 - room) * 2 + room_height
            }
        }
        4 => {
            if room > 2 {
                2 + (room - 3) * 2 + room_height
            } else {
                2 + (2 - room) * 2 + room_height
            }
        }
        5 => 2 + (3 - room) * 2 + room_height,
        6 => 3 + (3 - room) * 2 + room_height,
        _ => panic!(),
    }
}

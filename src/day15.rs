use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

use crate::common;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    idx: usize,
    fscore: usize,
}

impl Node {
    fn new(idx: usize, fscore: usize) -> Self {
        Node { idx, fscore }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.fscore.partial_cmp(&self.fscore)
    }

    fn lt(&self, other: &Self) -> bool {
        self.fscore >= other.fscore
    }

    fn le(&self, other: &Self) -> bool {
        self.fscore > other.fscore
    }

    fn gt(&self, other: &Self) -> bool {
        self.fscore <= other.fscore
    }

    fn ge(&self, other: &Self) -> bool {
        self.fscore < other.fscore
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fscore.cmp(&other.fscore)
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let lines = common::read_lines("inputs/15.txt")?;

    let mut map: Vec<usize> = Vec::new();
    let mut size_x = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let start = map.len();

        for c in line.chars() {
            let val = c as usize - '0' as usize;
            map.push(val);
        }
        size_x = line.len();
        if do_b {
            for i in 1..5 {
                for j in 0..size_x {
                    let val = map[start + j] + i;
                    if val > 9 {
                        map.push(val % 10 + 1);
                    } else {
                        map.push(val);
                    }
                }
            }
        }
    }

    if do_b {
        let length = map.len();
        map.reserve(length * 4);
        for i in 1..5 {
            for j in 0..length {
                let val = map[j] + i;
                if val > 9 {
                    map.push(val % 10 + 1);
                } else {
                    map.push(val);
                }
            }
        }
        size_x *= 5;
    }
    let size_y = map.len() / size_x;

    let goal = map.len() - 1;

    // Thank you wikipedia for the pseudo code :)

    let mut open_set = BinaryHeap::new();
    open_set.push(Node::new(0, 0));

    let mut gscore = vec![usize::MAX; size_x * size_y];
    gscore[0] = 0;

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().idx;

        if current == goal {
            break;
        }

        let x = current % size_x;
        let y = current / size_x;

        if x > 0 {
            let neighbour = current - 1;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                open_set.push(Node::new(
                    neighbour,
                    tentative_gscore + size_x - x + size_y - y,
                ));
            }
        }
        if x < size_x - 1 {
            let neighbour = current + 1;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                open_set.push(Node::new(
                    neighbour,
                    tentative_gscore + size_x - x + size_y - y,
                ));
            }
        }

        if y > 0 {
            let neighbour = current - size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                open_set.push(Node::new(
                    neighbour,
                    tentative_gscore + size_x - x + size_y - y,
                ));
            }
        }
        if y < size_y - 1 {
            let neighbour = current + size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                open_set.push(Node::new(
                    neighbour,
                    tentative_gscore + size_x - x + size_y - y,
                ));
            }
        }
    }
    Ok(gscore[goal])
}

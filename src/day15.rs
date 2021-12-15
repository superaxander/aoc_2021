use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap};
use std::io;

use crate::common;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    idx: usize,
    fscore: usize,
}

impl Node {
    fn new(idx: usize, fscore:usize) -> Self {
        Node {
            idx, fscore
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fscore.partial_cmp(&other.fscore)
    }

    fn lt(&self, other: &Self) -> bool {
        self.fscore < other.fscore
    }

    fn le(&self, other: &Self) -> bool {
        self.fscore <= other.fscore
    }

    fn gt(&self, other: &Self) -> bool {
        self.fscore > other.fscore
    }

    fn ge(&self, other: &Self) -> bool {
        self.fscore >= other.fscore
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
    open_set.push(Reverse(Node::new(0, 0)));

    let mut gscore = vec![usize::MAX; size_x * size_y];
    let mut fscore = vec![usize::MAX; size_x * size_y];
    gscore[0] = 0;
    fscore[0] = 0;

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0.idx;
        if current == goal {
            break;
        }

        let x = current % size_x;
        let y = current / size_x;

        if x > 0 {
            let neighbour = x - 1 + y * size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                fscore[neighbour] = tentative_gscore + (x + y) * (x + y);
                if !open_set.iter().any(|n| n.0.idx == neighbour) {
                    open_set.push(Reverse(Node::new(neighbour, fscore[neighbour])))
                }
            }
        }
        if x < size_x - 1 {
            let neighbour = x + 1 + y * size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                fscore[neighbour] = tentative_gscore + (x + y) * (x + y);
                if !open_set.iter().any(|n| n.0.idx == neighbour) {
                    open_set.push(Reverse(Node::new(neighbour, fscore[neighbour])))
                }
            }
        }

        if y > 0 {
            let neighbour = x + (y - 1) * size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                fscore[neighbour] = tentative_gscore + (x + y) * (x + y);
                if !open_set.iter().any(|n| n.0.idx == neighbour) {
                    open_set.push(Reverse(Node::new(neighbour, fscore[neighbour])))
                }
            }
        }
        if y < size_y - 1 {
            let neighbour = x + (y + 1) * size_x;
            let tentative_gscore = gscore[current] + map[neighbour];
            if tentative_gscore < gscore[neighbour] {
                gscore[neighbour] = tentative_gscore;
                fscore[neighbour] = tentative_gscore + (x + y) * (x + y);
                if !open_set.iter().any(|n| n.0.idx == neighbour) {
                    open_set.push(Reverse(Node::new(neighbour, fscore[neighbour])))
                }
            }
        }
    }

    Ok(gscore[goal])
}

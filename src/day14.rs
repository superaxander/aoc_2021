use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::io;

use crate::common;

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/14.txt")?;

    let mut polymer: HashMap<(char, char), usize> = HashMap::new();
    let mut insertion_pairs = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if polymer.is_empty() {
            let mut last_char = '\0';
            for (i, c) in line.chars().enumerate() {
                if i == 0 {
                    last_char = c;
                    continue;
                }
                match polymer.entry((last_char, c)) {
                    Occupied(mut e) => {
                        *e.get_mut() += 1;
                    }
                    Vacant(e) => {
                        e.insert(1);
                    }
                }
                last_char = c;
            }
        } else if !line.is_empty() {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            insertion_pairs.insert(
                (
                    split[0].chars().next().unwrap(),
                    split[0].chars().nth(1).unwrap(),
                ),
                split[1].chars().next().unwrap(),
            );
        }
    }
    let mut count_a = 0;
    for i in 0..40 {
        debug!("{:?}", polymer);
        let keys: HashMap<(char, char), usize> = polymer.clone();
        for (p, i) in keys {
            if let Some(r) = insertion_pairs.get(&p) {
                match polymer.entry((p.0, *r)) {
                    Occupied(mut e) => {
                        *e.get_mut() += i;
                    }
                    Vacant(e) => {
                        e.insert(i);
                    }
                }
                match polymer.entry((*r, p.1)) {
                    Occupied(mut e) => {
                        *e.get_mut() += i;
                    }
                    Vacant(e) => {
                        e.insert(i);
                    }
                }
                *polymer.get_mut(&p).unwrap() -= i;
            }
        }

        if i == 9 {
            let mut counts: HashMap<char, usize> = HashMap::new();
            polymer.keys().for_each(|s| {
                let i = polymer[s];
                match counts.entry(s.0) {
                    Occupied(mut e) => {
                        *e.get_mut() += i;
                    }
                    Vacant(e) => {
                        e.insert(i);
                    }
                };
            });
            count_a = counts.values().max().unwrap() - counts.values().min().unwrap() + 1;
        }
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    polymer.keys().for_each(|s| {
        let i = polymer[s];
        match counts.entry(s.0) {
            Occupied(mut e) => {
                *e.get_mut() += i;
            }
            Vacant(e) => {
                e.insert(i);
            }
        };
    });

    Ok((
        count_a,
        counts.values().max().unwrap() - counts.values().min().unwrap() + 1,
    ))
}

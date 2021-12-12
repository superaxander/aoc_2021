use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io;

use crate::common;

fn dfs<'a>(paths: &'a HashMap<String, Vec<String>>, visited: &'a HashSet<&'a str>, current: &'a str, allow_double: bool) -> (usize, usize) {
    let mut single_paths = 0;
    let mut double_paths = 0;
    for outgoing in &paths[current] {
        if outgoing == "end" {
            single_paths += 1;
        } else if !outgoing.chars().next().unwrap().is_lowercase() || !visited.contains(&**outgoing) {
            if paths.contains_key(outgoing) {
                let mut copy = visited.clone();
                copy.insert(outgoing);
                let (single, double) = dfs(paths, &copy, outgoing, allow_double);
                single_paths += single;
                double_paths += double;
            }
        } else if allow_double {
            if paths.contains_key(outgoing) {
                let (single, double) = dfs(paths, visited, outgoing, false);
                double_paths += single + double;
            }
        }
    }

    (single_paths, double_paths)
}

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let split: Vec<&str> = line.split('-').collect();
        let a = String::from(split[0]);
        let b = String::from(split[1]);
        if b != "start" {
            match paths.entry(a.clone()) {
                Entry::Occupied(mut e) => { e.get_mut().push(b.clone()); }
                Entry::Vacant(e) => { e.insert(vec![b.clone()]); }
            }
        }
        if a != "start" {
            match paths.entry(b) {
                Entry::Occupied(mut e) => { e.get_mut().push(a); }
                Entry::Vacant(e) => { e.insert(vec![a]); }
            }
        }
    }

    let mut set = HashSet::new();
    set.insert("start");
    let (single, double) = dfs(&paths, &set, "start", true);

    Ok((single, single + double))
}

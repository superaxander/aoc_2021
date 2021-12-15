use std::io;

use crate::common;

fn dfs(
    paths: &[Vec<usize>],
    big: &[bool],
    not_visited: &mut [bool],
    current: usize,
    allow_double: bool,
) -> (usize, usize) {
    let mut single_paths = 0;
    let mut double_paths = 0;
    for outgoing in &paths[current] {
        if *outgoing == 1 {
            single_paths += 1;
        } else if big[*outgoing] || not_visited[*outgoing] {
            not_visited[*outgoing] = false;
            let (single, double) = dfs(paths, big, not_visited, *outgoing, allow_double);
            not_visited[*outgoing] = true;
            single_paths += single;
            double_paths += double;
        } else if allow_double {
            let (single, double) = dfs(paths, big, not_visited, *outgoing, false);
            double_paths += single + double;
        }
    }

    (single_paths, double_paths)
}

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut paths: Vec<Vec<usize>> = vec![Vec::new(), Vec::new()];

    let mut cave_set = vec!["start".to_string(), "end".to_string()];

    for line in lines {
        let line = line?;
        let line = line.trim();
        let split: Vec<&str> = line.split('-').collect();

        let a;
        if let Some(found) = cave_set.iter().position(|c| *c == split[0]) {
            a = found;
        } else {
            a = cave_set.len();
            cave_set.push(split[0].to_string());
            paths.push(Vec::new())
        }

        let b;
        if let Some(found) = cave_set.iter().position(|c| *c == split[1]) {
            b = found;
        } else {
            b = cave_set.len();
            cave_set.push(split[1].to_string());
            paths.push(Vec::new())
        }

        if b > 0 {
            paths[a].push(b);
        }
        if a > 0 {
            paths[b].push(a);
        }
    }

    let cave_count = cave_set.len();

    let mut set = vec![true; cave_count];
    set[0] = false;
    let (single, double) = dfs(
        &paths,
        &cave_set
            .iter()
            .map(|s| !s.chars().next().unwrap().is_lowercase())
            .collect::<Vec<bool>>(),
        &mut set,
        0,
        true,
    );

    Ok((single, single + double))
}

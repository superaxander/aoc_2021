use std::collections::HashSet;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let lines = common::read_lines("inputs/8.txt")?;

    let mut count = 0;

    let mut known_patterns = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    let mut unknown_patterns = Vec::new();
    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.trim().split(" | ").collect();
        let output_values: Vec<&str> = split[1].split(' ').collect();
        if do_b {
            let unique_patterns: Vec<&str> = split[0].split(' ').collect();
            /*
              000
             1   2
             1   2
              333
             4   5
             4   5
              666 
             */
            for unique_pattern in unique_patterns {
                match unique_pattern.len() {
                    2 => {
                        unique_pattern.chars().for_each(|c| { known_patterns[1].insert(c); });
                    }
                    4 => {
                        unique_pattern.chars().for_each(|c| { known_patterns[4].insert(c); });
                    }
                    3 => {
                        unique_pattern.chars().for_each(|c| { known_patterns[7].insert(c); });
                    }
                    7 => {
                        unique_pattern.chars().for_each(|c| { known_patterns[8].insert(c); });
                    }
                    _ => {
                        unknown_patterns.push(unique_pattern.chars().collect::<HashSet<char>>())
                    }
                }
            }
            
            unknown_patterns.retain(|unknown_pattern| if unknown_pattern.len() == 6 {
                if (&known_patterns[4]).is_subset(unknown_pattern) {
                    (&mut known_patterns[9]).extend(unknown_pattern);
                    false
                } else if (&known_patterns[7]).is_subset(unknown_pattern) {
                    (&mut known_patterns[0]).extend(unknown_pattern);
                    false
                } else {
                    (&mut known_patterns[6]).extend(unknown_pattern);
                    false
                }
            } else if unknown_pattern.len() == 5 {
                if (&known_patterns[7]).is_subset(unknown_pattern) {
                    (&mut known_patterns[3]).extend(unknown_pattern);
                    false
                } else {
                    true
                }
            } else {
                true
            });
            assert!(unknown_patterns.len() == 2);
            if unknown_patterns[0] == (&known_patterns[9]).intersection(&unknown_patterns[0]).copied().collect::<HashSet<char>>() {
                (&mut known_patterns[5]).extend(&unknown_patterns[0]);
                (&mut known_patterns[2]).extend(&unknown_patterns[1]);
            } else {
                (&mut known_patterns[2]).extend(&unknown_patterns[0]);
                (&mut known_patterns[5]).extend(&unknown_patterns[1]);
            }
            
            let mut ov_sum:usize = 0;
            
            for ov in output_values {
                let set: HashSet<char> = ov.chars().collect();
                for (i, p) in known_patterns.iter().enumerate() {
                    if p == &set {
                        ov_sum = 10 * ov_sum + i;
                        break;
                    }
                }
            }
            count += ov_sum;
            
            for p in &mut known_patterns {
                p.clear();
            }
            
            unknown_patterns.clear();
        } else {
            for output_value in output_values {
                match output_value.len() {
                    2 | 4 | 3 | 7 => count += 1,
                    _ => {}
                }
            }
        }
    }


    Ok(count)
}

use std::collections::HashSet;
use std::io;

use crate::common;

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn intersections(&self, other: &Line) -> Vec<(i32, i32)> {
        macro_rules! do_loop {
            ($var_x:ident, $delta_var_x:ident, $start_x:expr, $end_x:expr, $var_y:ident, $delta_var_y:ident, $start_y:expr, $end_y:expr, $action:block) => { 
                let $delta_var_x = $start_x - $end_x;
                let $delta_var_y = $start_y - $end_y;
                let mut $var_x = $start_x;
                let mut $var_y = $start_y;
                if $delta_var_x > 0 {
                    if $delta_var_y > 0 {
                        while $var_x >= $end_x && $var_y >= $end_y{ 
                        
                            $action
                        
                            $var_x -= 1;
                            $var_y -= 1;
                        }
                    } else if $delta_var_y == 0 {
                        while $var_x >= $end_x {
                        
                            $action
                        
                            $var_x -= 1;
                        }
                    } else {
                        while $var_x >= $end_x && $var_y <= $end_y{ 
                        
                            $action
                        
                            $var_x -= 1;
                            $var_y += 1;
                        }
                    }
                } else if $delta_var_x == 0 {
                    if $delta_var_y > 0 {
                        while $var_y >= $end_y{ 
                        
                            $action
                            
                            $var_y -= 1;
                        }
                    } else if $delta_var_y == 0 {
                        $action
                    } else {
                        while $var_y <= $end_y{
                            $action
                            
                            $var_y += 1;
                        }
                    }
                } else {
                    if $delta_var_y > 0 {
                        while $var_x <= $end_x && $var_y >= $end_y{ 
                        
                            $action
                        
                            $var_x += 1;
                            $var_y -= 1;
                        }
                    } else if $delta_var_y == 0 {
                        while $var_x <= $end_x {
                        
                            $action
                        
                            $var_x += 1;
                        }
                    } else {
                        while $var_x <= $end_x && $var_y <= $end_y{ 
                        
                            $action
                        
                            $var_x += 1;
                            $var_y += 1;
                        }
                    }
                }
            };
        }
        let mut intersection_points = Vec::new();

        do_loop!(x1, delta_x1, self.x1, self.x2, y1, delta_y1, self.y1, self.y2, {
            do_loop!(x2, delta_x2, other.x1, other.x2, y2, delta_y2, other.y1, other.y2, {
                if x1 == x2 && y1 == y2 {
                    intersection_points.push((x1, y1));
                }
            });
        });


        intersection_points
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut vent_lines = Vec::new();

    let lines = common::read_lines("inputs/5.txt")?;
    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.trim().split(" -> ").collect();
        assert_eq!(split.len(), 2);
        let line_start: Vec<&str> = split[0].split(",").collect();
        let line_end: Vec<&str> = split[1].split(",").collect();
        vent_lines.push(Line {
            x1: line_start[0].parse().unwrap(),
            y1: line_start[1].parse().unwrap(),
            x2: line_end[0].parse().unwrap(),
            y2: line_end[1].parse().unwrap(),
        })
    }

    if !do_b {
        vent_lines.retain(|line| line.x1 == line.x2 || line.y1 == line.y2);
    }

    let mut set = HashSet::new();

    for i in 0..vent_lines.len() {
        let line_a = &vent_lines[i];
        for j in i + 1..vent_lines.len() {
            set.extend(line_a.intersections(&vent_lines[j]));
            // debug!("{:?} and {:?} intersect {} times", line_a, vent_lines[j], line_a.intersections(&vent_lines[j]));
        }
    }

    return Ok(set.len());
}

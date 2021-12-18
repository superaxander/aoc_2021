use std::fmt::{Debug, Formatter};
use std::io;
use std::ops::Add;

use crate::common;
use crate::day18::Number::{Literal, Pair};

#[derive(Eq, PartialEq, Clone)]
enum Number {
    Pair(Box<Number>, Box<Number>),
    Literal(usize),
}

impl Number {
    fn first(&mut self) -> &mut usize {
        match self {
            Pair(l, _) => l.first(),
            Literal(n) => n
        }
    }
    
    fn last(&mut self) -> &mut usize {
        match self {
            Pair(_, r) => r.last(),
            Literal(n) => n
        }
    }
    
    fn split(self) -> (bool, Box<Number>) {
        match self {
            Pair(l, r) => {
                let (split, l) = l.split();
                if split {
                    (true, Box::new(Pair(l, r)))
                }else {
                    let (split, r) = r.split();
                    (split, Box::new(Pair(l, r)))
                }
            }
            Literal(n) => if n > 9 {
                (true, Box::new(Pair(Box::new(Literal(n / 2)), Box::new(Literal((n + 1) / 2)))))
            } else {
                (false, Box::new(self))
            }
        }
    }

    fn explode(self, depth: usize, mut left_num: Option<&mut usize>, right_num: Option<&mut usize>) -> (bool, Box<Number>) {
        if depth == 3 {
            match self {
                Pair(mut l, mut r) => {
                    match (&mut *l, &mut *r) {
                        (Pair(ll, lr), Literal(rn)) => {
                            if let Literal(ll) = **ll {
                                if let Literal(lr) = **lr {
                                    if let Some(l) = left_num {
                                        *l += ll;
                                    }
                                    *rn += lr;
                                    (true, Box::new(Pair(Box::new(Literal(0)), r)))
                                } else {
                                    panic!("Impossible pattern")
                                }
                            } else {
                                panic!("Impossible pattern")
                            }
                        }
                        (Literal(ln), Pair(rl, rr)) => {
                            if let Literal(rl) = **rl {
                                if let Literal(rr) = **rr {
                                    *ln += rl;
                                    if let Some(r) = right_num {
                                        *r += rr;
                                    }
                                    (true, Box::new(Pair(l, Box::new(Literal(0)))))
                                } else {
                                    panic!("Impossible pattern")
                                }
                            } else {
                                panic!("Impossible pattern")
                            }
                        }
                        (Pair(ll, lr), Pair(rl, _)) => {
                            if let Literal(ll) = **ll {
                                if let Literal(lr) = **lr {
                                    if let Some(l) = &mut left_num {
                                        **l += ll;
                                    }
                                    if let Literal(rl) = &mut **rl {
                                        *rl += lr;
                                    }
                                    (true, (Pair(Box::new(Literal(0)), r).explode(depth, left_num, right_num)).1)
                                } else {
                                    panic!("Impossible pattern")
                                }
                            } else {
                                panic!("Impossible pattern")
                            }
                        }
                        _ => {
                            (false, Box::new(Pair(l, r)))
                        }
                    }
                }
                _ => (false, Box::new(self))
            }
        } else {
            match self {
                Pair(l, mut r) => {
                    let (changed, l) = l.explode(depth + 1, left_num, Some(r.first()));
                    let mut l = *l;
                    let left_num = l.last();
                    let (c, r) = r.explode(depth + 1, Some(left_num), right_num);
                    (changed || c, Box::new(Pair(Box::new(l), r)))
                }
                _ => (false, Box::new(self))
            }
        }
    }
    
    fn magnitude(self) -> usize {
        match self {
            Pair(l, r) => 3 * l.magnitude() + 2*r.magnitude(),
            Literal(n) => n
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair(l, r) => f.write_fmt(format_args!("[{:?},{:?}]", l, r)),
            Literal(n) => f.write_fmt(format_args!("{}", n))
        }
    }
}

impl Add for Box<Number> {
    type Output = Box<Number>;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(Pair(self, rhs))
    }
}

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/18.txt")?;

    let mut current = Box::new(Literal(0));
    let mut pairs = Vec::new();


    for line in lines {
        let line = line?;
        let line = line.trim();

        let pair = parse_pair(&line.chars().collect::<Vec<char>>()).1;
        pairs.push(pair.clone());

        if *current == Literal(0) {
            current = pair;
        } else {
            current = reduce(current + pair)
        }
    }
    
    let mut max = 0;
    for i in 0..pairs.len() {
        for j in i+1..pairs.len() {
            let magnitude = reduce(pairs[i].clone()+pairs[j].clone()).magnitude();
            if magnitude > max {
                max = magnitude;
            }

            let magnitude = reduce(pairs[j].clone() + pairs[i].clone()).magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }
    }

    Ok((current.magnitude(), max))
}

fn reduce(current: Box<Number>) -> Box<Number>{
    let mut current = current;
    loop {
        loop {
            let (exploded, new) = current.explode(0, None, None);
            current = new;
            if !exploded { break; }
        }
        let (split, new) = current.split();
        current = new;
        if !split { break; }
    }
    current
}

fn parse_pair(str: &[char]) -> (usize, Box<Number>) {
    if str[0] == '[' {
        let mut idx = 1;
        let (i, left) = parse_pair(&str[idx..]);
        idx += i + 1;
        let (i, right) = parse_pair(&str[idx..]);
        (idx + i + 1, Box::new(Pair(left, right)))
    } else {
        (1, Box::new(Literal(str[0] as usize - '0' as usize)))
    }
}

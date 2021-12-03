use std::io;
use std::ops::Shr;

use crate::common;

fn part_a() -> io::Result<usize> {
    let lines = common::read_lines("inputs/3.txt")?;

    let mut zeroes: Vec<usize> = Vec::new();
    let mut ones: Vec<usize> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if zeroes.len() == 0 {
            for _ in 0..line.len() {
                zeroes.push(0);
                ones.push(0);
            }
        }
        let mut i = 0;
        for c in line.chars() {
            match c {
                '1' => {
                    ones[i] += 1;
                }
                '0' => {
                    zeroes[i] += 1;
                }
                _ => debug!("error in input invalid character: {}", c)
            }
            i += 1;
        }
    }

    let mut gamma = 0;
    let length = zeroes.len();
    for i in 0..length {
        if zeroes[i] > ones[i] {
            gamma = gamma * 2;
        } else {
            gamma = gamma * 2 + 1;
        }
    }

    let mask = (0xFFFFFFFFusize).shr(32 - length);

    return Ok(gamma * (!gamma & mask));
}

fn part_b() -> io::Result<usize> {
    let lines = common::read_lines("inputs/3.txt")?;

    let mut oxygen_nums = Vec::new();
    let mut length = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        
        let mut num = 0;
        for c in line.chars() {
            match c {
                '1' => num = 1 + num * 2,
                '0' => num = num * 2,
                _ => debug!("error in input invalid character: {}", c)
            }
        }
        length = line.len();
        oxygen_nums.push(num);
    }

    let mut scrubber_nums = oxygen_nums.clone();
    let mut oxygen_rating = -1;
    let mut scrubber_rating = -1;
    for i in 0..length {
        let shift_param = length - i - 1;

        if is_zero_most_common(&oxygen_nums, shift_param) {
            oxygen_nums.retain(|n| (n.shr(shift_param) & 0b1) == 0);
        } else {
            oxygen_nums.retain(|n| (n.shr(shift_param) & 0b1) == 1);
        }

        if oxygen_nums.len() == 1 {
            oxygen_rating = oxygen_nums[0];
            if scrubber_rating != -1 {
                break;
            }
        }

        if is_zero_most_common(&scrubber_nums, shift_param) {
            scrubber_nums.retain(|n| (n.shr(shift_param) & 0b1) == 1);
        } else {
            scrubber_nums.retain(|n| (n.shr(shift_param) & 0b1) == 0);
        }

        if scrubber_nums.len() == 1 {
            scrubber_rating = scrubber_nums[0];
            if oxygen_rating != -1 {
                break;
            }
        }
    }

    return Ok((oxygen_rating * scrubber_rating) as usize);
}

fn is_zero_most_common(nums: &Vec<i32>, shift_param: usize) -> bool {
    let mut zeroes = 0;
    let mut ones = 0;

    for n in nums {
        if (n.shr(shift_param) & 1) == 0 {
            zeroes += 1;
        } else {
            ones += 1;
        }
    }

    zeroes > ones
}


pub fn main(do_b: bool) -> io::Result<usize> {
    return if do_b {
        part_b()
    } else {
        part_a()
    };
}

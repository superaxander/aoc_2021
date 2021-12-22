use std::io;

use crate::common;

type Region = (i64, i64, i64, i64, i64, i64);

fn get_remaining_overlap(region: &Region, exceptions: &[Region]) -> usize {
    let mut size = size(region);

    let mut processed_exceptions = Vec::new();

    for exception in exceptions {
        if let Some(overlap) = find_overlap(region, exception) {
            let remaining_overlap = get_remaining_overlap(&overlap, &processed_exceptions);

            if remaining_overlap > size {
                return 0;
            }
            size -= remaining_overlap;
            processed_exceptions.push(overlap);
        }
    }

    size
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let lines = common::read_lines("inputs/22.txt")?;

    let mut on_regions: Vec<Region> = Vec::new();
    let mut exceptions: Vec<Vec<Region>> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let split = line.split(' ').collect::<Vec<&str>>();
        let is_on = split[0] == "on";
        let split = split[1].split(',').collect::<Vec<&str>>();
        let x_split = split[0].split("..").collect::<Vec<&str>>();
        let x_min = x_split[0][2..].parse::<i64>().unwrap();
        let x_max = x_split[1].parse::<i64>().unwrap();
        let y_split = split[1].split("..").collect::<Vec<&str>>();
        let y_min = y_split[0][2..].parse::<i64>().unwrap();
        let y_max = y_split[1].parse::<i64>().unwrap();
        let z_split = split[2].split("..").collect::<Vec<&str>>();
        let z_min = z_split[0][2..].parse::<i64>().unwrap();
        let z_max = z_split[1].parse::<i64>().unwrap();
        let region = (x_min, x_max, y_min, y_max, z_min, z_max);
        if !do_b && (x_min.min(y_min.min(z_min)) < -50 || x_max.max(y_max.max(z_max)) > 50) {
            break;
        }
        if is_on {
            let overlaps = Vec::new();

            for (i, on_region) in on_regions.iter().enumerate() {
                if check_overlap(&region, on_region) > 0 {
                    exceptions[i].push(region);
                }
            }

            on_regions.push(region);
            exceptions.push(overlaps);
        } else {
            for (i, on_region) in on_regions.iter().enumerate() {
                if check_overlap(on_region, &region) > 0 {
                    exceptions[i].push(region);
                }
            }
        }
    }

    let mut count = 0;
    for (i, on_region) in on_regions.iter().enumerate() {
        let exceptions = &exceptions[i];

        let additional_size = get_remaining_overlap(on_region, exceptions);

        count += additional_size;
    }

    Ok(count)
}

fn check_overlap(r0: &Region, r1: &Region) -> usize {
    if r0.1 >= r1.0 && r0.0 <= r1.1 && r0.3 >= r1.2 && r0.2 <= r1.3 && r0.5 >= r1.4 && r0.4 <= r1.5
    {
        let x_overlap = r0.1.min(r1.1) - r0.0.max(r1.0) + 1;
        let y_overlap = r0.3.min(r1.3) - r0.2.max(r1.2) + 1;
        let z_overlap = r0.5.min(r1.5) - r0.4.max(r1.4) + 1;
        (x_overlap * y_overlap * z_overlap) as usize
    } else {
        0
    }
}

fn find_overlap(r0: &Region, r1: &Region) -> Option<Region> {
    if r0.1 >= r1.0 && r0.0 <= r1.1 && r0.3 >= r1.2 && r0.2 <= r1.3 && r0.5 >= r1.4 && r0.4 <= r1.5
    {
        Some((
            r0.0.max(r1.0),
            r0.1.min(r1.1),
            r0.2.max(r1.2),
            r0.3.min(r1.3),
            r0.4.max(r1.4),
            r0.5.min(r1.5),
        ))
    } else {
        None
    }
}

fn size(region: &Region) -> usize {
    ((region.1 - region.0 + 1) * (region.3 - region.2 + 1) * (region.5 - region.4 + 1)) as usize
}

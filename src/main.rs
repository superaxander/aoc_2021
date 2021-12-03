#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use common::{Day, Runnable};
use std::time::Instant;

mod common;
mod day1;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
mod day2;
mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;

fn main() {
    pretty_env_logger::init();
    let now = Instant::now();
    Day::Combined(day1::main).run("day 1");
    Day::Combined(day2::main).run("day 2");
    Day::SeparatedUsize(day3::main).run("day 3");
    // Day::Separated(day4::main).run("day 4");
    // Day::Separated(day5::main).run("day 5");
    // Day::SeparatedUsize(day6::main).run("day 6");
    // Day::SeparatedUsize(day7::main).run("day 7");
    // Day::Separated(day8::main).run("day 8");
    // Day::CombinedUsize(day9::main).run("day 9");
    // Day::SeparatedUsize(day10::main).run("day 10");
    // Day::SeparatedUsize(day11::main).run("day 11");
    // Day::Separated(day12::main).run("day 12");
    // Day::SeparatedULong(day13::main).run("day 13");
    // Day::SeparatedULong(day14::main).run("day 14");
    // Day::SeparatedUsize(day15::main).run("day 15");
    // Day::CombinedUsize(day16::main).run("day 16");
    info!("All days together took {:#?}", now.elapsed());
}

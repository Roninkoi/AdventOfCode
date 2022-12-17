#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_mut)]
use std::env;
use std::time::{Duration, Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);

    macro_rules! d1 {
        () => {
            println!("=== Day 1, part 1 ===");
            day1::day1_1("input/day1_input");
            println!("=== Day 1, part 2 ===");
            day1::day1_2("input/day1_input");
        };
    }
    macro_rules! d2 {
        () => {
            println!("=== Day 2, part 1 ===");
            day2::day2_1("input/day2_input");
            println!("=== Day 2, part 2 ===");
            day2::day2_2("input/day2_input");
        };
    }
    macro_rules! d3 {
        () => {
            println!("=== Day 3, part 1 ===");
            day3::day3_1("input/day3_input");
            println!("=== Day 3, part 2 ===");
            day3::day3_2("input/day3_input");
        };
    }
    macro_rules! d4 {
        () => {
            println!("=== Day 4, part 1 ===");
            day4::day4_1("input/day4_input");
            println!("=== Day 4, part 2 ===");
            day4::day4_2("input/day4_input");
        };
    }
    macro_rules! d5 {
        () => {
            println!("=== Day 5, part 1 ===");
            day5::day5_1("input/day5_input");
            println!("=== Day 5, part 2 ===");
            day5::day5_2("input/day5_input");
        };
    }
    macro_rules! d6 {
        () => {
            println!("=== Day 6, part 1 ===");
            day6::day6_1("input/day6_input");
            println!("=== Day 6, part 2 ===");
            day6::day6_2("input/day6_input");
        };
    }
    macro_rules! d7 {
        () => {
            println!("=== Day 7, part 1 ===");
            day7::day7_1("input/day7_input");
            println!("=== Day 7, part 2 ===");
            day7::day7_2("input/day7_input");
        };
    }
    macro_rules! d8 {
        () => {
            println!("=== Day 8, part 1 ===");
            day8::day8_1("input/day8_input");
            println!("=== Day 8, part 2 ===");
            day8::day8_2("input/day8_input");
        };
    }
    macro_rules! d9 {
        () => {
            println!("=== Day 9, part 1 ===");
            day9::day9_1("input/day9_input");
            println!("=== Day 9, part 2 ===");
            day9::day9_2("input/day9_input");
        };
    }
    macro_rules! d10 {
        () => {
            println!("=== Day 10 ===");
            day10::day10("input/day10_input");
        };
    }
    macro_rules! d11 {
        () => {
            println!("=== Day 11, part 1 ===");
            day11::day11_1("input/day11_input");
            println!("=== Day 11, part 2 ===");
            day11::day11_2("input/day11_input");
        };
    }
    macro_rules! d12 {
        () => {
            println!("=== Day 12, part 1 ===");
            day12::day12_1("input/day12_input");
            println!("=== Day 12, part 2 ===");
            day12::day12_2("input/day12_input");
        };
    }
    macro_rules! d13 {
        () => {
            println!("=== Day 13, part 1 ===");
            day13::day13_1("input/day13_ex");
            /*println!("=== Day 13, part 2 ===");
            day13::day13_2("input/day13_input");*/
        };
    }

    if day.is_none() {
        d1!();
        d2!();
        d3!();
        d4!();
        d5!();
        d6!();
        d7!();
        d8!();
        d9!();
        d10!();
        d11!();
        d12!();
        d13!();
        return;
    }

    match day.unwrap().as_str() {
        "day1" => { d1!(); }
        "day2" => { d2!(); }
        "day3" => { d3!(); }
        "day4" => { d4!(); }
        "day5" => { d5!(); }
        "day6" => { d6!(); }
        "day7" => { d7!(); }
        "day8" => { d8!(); }
        "day9" => { d9!(); }
        "day10" => { d10!(); }
        "day11" => { d11!(); }
        "day12" => { d12!(); }
        "bench" => {
            let mut diffs: Vec<Duration> = Vec::new();
            let mut start = Instant::now();
            let mut measure = || {
                diffs.push(start.elapsed());
                start = Instant::now();
            };
            d1!();
            measure();
            d2!();
            measure();
            d3!();
            measure();
            d4!();
            measure();
            d5!();
            measure();
            d6!();
            measure();
            d7!();
            measure();
            d8!();
            measure();
            d9!();
            measure();
            d10!();
            measure();
            d11!();
            measure();
            d12!();
            measure();

            for (i, diff) in diffs.iter().enumerate() {
                println!("Day {} time: {} us", i + 1, diff.as_micros());
            }
            println!("Total time: {} ms", diffs.iter().map(|d| d.as_micros()).sum::<u128>() / 1000);
        }
        _ => {}
    }
}

#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_mut)]
use std::env;
use std::time::{Duration, Instant};

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    let mut printing = true;

    macro_rules! d1 {
        () => {
            day1::day1_1("input/day1_input");
            day1::day1_2("input/day1_input");
        };
    }
    macro_rules! d2 {
        () => {
            day2::day2_1("input/day2_input");
            day2::day2_2("input/day2_input");
        };
    }
    macro_rules! d3 {
        () => {
            day3::day3_1("input/day3_input");
            day3::day3_2("input/day3_input");
        };
    }
    macro_rules! d4 {
        () => {
            day4::day4_1("input/day4_input");
            day4::day4_2("input/day4_input");
        };
    }
    macro_rules! d5 {
        () => {
            day5::day5_1("input/day5_input");
            day5::day5_2("input/day5_input");
        };
    }
    macro_rules! d6 {
        () => {
            day6::day6_1("input/day6_input");
            day6::day6_2("input/day6_input");
        };
    }
    macro_rules! d7 {
        () => {
            day7::day7_1("input/day7_input");
            day7::day7_2("input/day7_input");
        };
    }
    macro_rules! d8 {
        () => {
            day8::day8_1("input/day8_input");
            day8::day8_2("input/day8_input");
        };
    }
    macro_rules! d9 {
        () => {
            day9::day9_1("input/day9_input");
            day9::day9_2("input/day9_input");
        };
    }
    macro_rules! d10 {
        () => {
            day10::day10("input/day10_input");
        };
    }
    macro_rules! d11 {
        () => {
            day11::day11_1("input/day11_input");
            day11::day11_2("input/day11_input");
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

            for (i, diff) in diffs.iter().enumerate() {
                println!("Day {} time: {} ms", i + 1, diff.as_millis());
            }
        }
        _ => {}
    }
}

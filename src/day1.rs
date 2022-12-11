use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::util::{dprintln, dprint};

pub fn day1_1(file_path: &str) {
    let stdin = io::stdin();
    let file = File::open(file_path).expect("File not found!");
    let reader = BufReader::new(file);

    let mut elf_calories = Vec::<i32>::new();
    let mut total_calories = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            let v = l.parse::<i32>().unwrap();
            total_calories += v;
        } else {
            elf_calories.push(total_calories);
            total_calories = 0;
        }
    }
    elf_calories.push(total_calories);

    let mut elf_max = 0;
    let mut cal_max = 0;
    for (elf, cal) in elf_calories.iter().enumerate() {
        dprintln!("{elf}, {cal}");
        if *cal > cal_max {
            cal_max = *cal;
            elf_max = elf;
        }
    }

    println!("Elf {elf_max} has the most calories: {cal_max}");
}

pub fn day1_2(file_path: &str) {
    let stdin = io::stdin();
    let file = File::open(file_path).expect("File not found!");
    let reader = BufReader::new(file);

    let mut elf_calories = Vec::<i32>::new();
    let mut total_calories = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            let v = l.parse::<i32>().unwrap();
            total_calories += v;
        } else {
            elf_calories.push(total_calories);
            total_calories = 0;
        }
    }
    elf_calories.push(total_calories);

    elf_calories.sort();
    elf_calories.reverse();

    let mut cal_sum = 0;
    for i in 0..3 {
        if i >= elf_calories.len() {
            break;
        }
        let cal = elf_calories[i];
        dprintln!("{i} {cal}");
        cal_sum += cal;
    }

    println!("Top 3 total calories: {cal_sum}");
}

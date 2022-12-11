use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

fn read_program(file_path: &str) -> Vec<(String, i32)> {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut program = Vec::<(String, i32)>::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();
        let words: Vec<&str> = l.split_whitespace().collect();
        let op = words[0].to_string();
        let mut arg = 0;
        if words.len() > 1 {
            arg = words[1].parse::<i32>().unwrap();
        }

        program.push((op, arg));
    }
    program
}

fn render(x: i32, cycle: i32) -> io::Result<()> {
    let r = (cycle - 1) % 40;
    if (r - (x + 1)).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }

    if (r == 0) {
        println!();
    }
    Ok(())
}

fn simulate_cpu(file_path: &str) -> io::Result<()> {
    let mut program = read_program(file_path);

    let mut cycle = 1;
    let mut x = 1;
    let mut signal: Vec<i32> = Vec::new();

    let mut tick = |reg: i32, cycle: &mut i32| {
        signal.push((reg) * (*cycle));
        *cycle += 1;
    };

    for (op, arg) in program {
        match op.as_str() {
            "noop" => {
                tick(x, &mut cycle);
                render(x, cycle);
            }
            "addx" => {
                tick(x, &mut cycle);
                render(x, cycle);
                tick(x, &mut cycle);
                render(x, cycle);
                x += arg;
            }
            _ => {}
        }
        //println!("{} {} {}", cycle, x, cycle * x);
    }

    let signal_range = (19..220).step_by(40);
    let signal_strengths = signal_range
        .map(|c| {
            let s = signal.get(c);
            if s.is_some() {
                *s.unwrap()
            } else {
                0
            }
        })
        .collect::<Vec<i32>>();
    println!(
        "{:?} {}",
        signal_strengths,
        signal_strengths.iter().sum::<i32>()
    );

    Ok(())
}

pub fn day10(file_path: &str) -> io::Result<()> {
    simulate_cpu(file_path)
}

use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::util::{dprintln, dprint};

fn read_motions(file_path: &str) -> Vec<(String, i32)> {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut motions = Vec::<(String, i32)>::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();
        let words: Vec<&str> = l.split_whitespace().collect();
        let dir = words[0].to_string();
        let steps = words[1].parse::<i32>().unwrap();

        motions.push((dir, steps));
    }
    motions
}

fn simulate_rope(file_path: &str, tail_len: usize) -> io::Result<()> {
    let mut motions = read_motions(file_path);
    let mut tail_path = HashSet::new();

    let mut step_i = 0;
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut head = (0, 0);
    let mut tail: Vec<(i32, i32)> = vec![(0, 0); tail_len];
    for m in motions {
        let dir = m.0;
        let mut steps = m.1;

        //println!("step {step_i}: move {dir} {steps}");
        while steps > 0 {
            match dir.as_str() {
                "D" => {
                    head.1 -= 1;
                }
                "U" => {
                    head.1 += 1;
                }
                "L" => {
                    head.0 -= 1;
                }
                "R" => {
                    head.0 += 1;
                }
                _ => {}
            };
            steps -= 1;

            x_range = (cmp::min(head.0, x_range.0), cmp::max(head.0, x_range.1));
            y_range = (cmp::min(head.1, y_range.0), cmp::max(head.1, y_range.1));

            let mut ps = &head;
            for ts in tail.iter_mut() {
                let x_diff = ps.0 - ts.0;
                let y_diff = ps.1 - ts.1;
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    if x_diff >= 1 {
                        ts.0 += 1;
                    } else if x_diff <= -1 {
                        ts.0 -= 1;
                    }
                    if y_diff >= 1 {
                        ts.1 += 1;
                    } else if y_diff <= -1 {
                        ts.1 -= 1;
                    }
                }
                ps = ts;
            }

            let last = tail.last().unwrap();
            tail_path.insert(format!("{}-{}", last.0, last.1));
        }

        step_i += 1;
    }

    for y in (y_range.0..y_range.1 + 1).rev() {
        for x in x_range.0..x_range.1 + 1 {
            let pos_str: String = format!("{}-{}", x, y);
            let pos = (x, y);
            if head == (x, y) {
                dprint!("H");
            } else if tail.contains(&pos) {
                let t: usize = tail
                    .iter()
                    .enumerate()
                    .filter(|(i, t)| **t == pos)
                    .map(|(i, t)| i)
                    .next()
                    .unwrap();
                dprint!("{}", t + 1);
            } else if (0, 0) == (x, y) {
                dprint!("s");
            } else if tail_path.contains(pos_str.as_str()) {
                dprint!("#");
            } else {
                dprint!(".");
            }
        }
        dprintln!();
    }
    dprintln!();

    let tail_pos_num = tail_path.len();
    println!("number of positions visited by tail: {tail_pos_num}");

    Ok(())
}

pub fn day9_1(file_path: &str) -> io::Result<()> {
    simulate_rope(file_path, 1)
}

pub fn day9_2(file_path: &str) -> io::Result<()> {
    simulate_rope(file_path, 9)
}

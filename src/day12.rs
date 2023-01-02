use regex;
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::util::{dprint, dprintln};

fn read_heights(file_path: &str) -> (Vec<Vec<i32>>, (usize, usize), (usize, usize)) {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut heights = Vec::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();

        if l.is_empty() {
            continue;
        }
        let to_height = |c: char| c as i32 - 'a' as i32;
        let h: Vec<i32> = l
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == 'E' {
                    end = (i, heights.len());
                    return to_height('z');
                } else if c == 'S' {
                    start = (i, heights.len());
                    return to_height('a');
                } else {
                    return to_height(c);
                }
            })
            .collect();
        heights.push(h);
    }
    (heights, start, end)
}

fn to_char(h: i32) -> char {
    (h as u8 + 'a' as u8) as char
}

fn print_heights(
    heights: &Vec<Vec<i32>>,
    pos: &(usize, usize),
    start: &(usize, usize),
    end: &(usize, usize),
    visited: &HashSet<(usize, usize)>,
) {
    for (i, row) in heights.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let ji = (j, i);
            if i == pos.1 && j == pos.0 {
                dprint!("#");
            } else if i == start.1 && j == start.0 {
                dprint!("S");
            } else if i == end.1 && j == end.0 {
                dprint!("E");
            } else if visited.contains(&ji) {
                dprint!("*");
            } else {
                dprint!("{}", to_char(*col));
            }
        }
        dprintln!();
    }
}

#[derive(Clone, Debug)]
struct Path {
    pos: (usize, usize),
    dist: u32,
}

pub fn day12_1(file_path: &str) -> io::Result<()> {
    let (heights, start, end) = read_heights(file_path);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut path: VecDeque<Path> = VecDeque::new();
    dprintln!(
        "start: ({}, {}) end: ({}, {})",
        start.0,
        start.1,
        end.0,
        end.1
    );
    let h = heights.len();
    let w = heights.first().unwrap().len();

    let mut step = 0;
    let mut shortest = 0;
    let mut pos = (0, 0);
    let mut dist = 0u32;
    path.push_back(Path {
        pos: start,
        dist: 0,
    });
    visited.insert(start);

    print_heights(&heights, &pos, &start, &end, &visited);
    loop {
        let this = path.pop_front();
        if this.is_some() {
            pos = this.clone().unwrap().pos;
            dist = this.clone().unwrap().dist;
        } else {
            println!("path ended!");
            break;
        }

        let mut r = (pos.0 /*+ 1*/, pos.1);
        let mut l = (pos.0 /*- 1*/, pos.1);
        let mut u = (pos.0, pos.1 /*- 1*/);
        let mut d = (pos.0, pos.1 /*+ 1*/);
        let mut right = if r.0 + 1 < w {
            r.0 += 1;
            heights[r.1][r.0]
        } else {
            100
        };
        let mut left = if l.0 >= 1 {
            l.0 -= 1;
            heights[l.1][l.0]
        } else {
            100
        };
        let mut up = if u.1 >= 1 {
            u.1 -= 1;
            heights[u.1][u.0]
        } else {
            100
        };
        let mut down = if d.1 + 1 < h {
            d.1 += 1;
            heights[d.1][d.0]
        } else {
            100
        };

        let h = heights[pos.1][pos.0];

        let eq_or_pm = |a: i32, b: i32| b - a <= 1;

        if eq_or_pm(h, right) && !visited.contains(&r) {
            visited.insert(r);
            path.push_back(Path {
                pos: r,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, left) && !visited.contains(&l) {
            visited.insert(l);
            path.push_back(Path {
                pos: l,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, up) && !visited.contains(&u) {
            visited.insert(u);
            path.push_back(Path {
                pos: u,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, down) && !visited.contains(&d) {
            visited.insert(d);
            path.push_back(Path {
                pos: d,
                dist: dist + 1,
            });
        }

        if pos.0 == end.0 && pos.1 == end.1 {
            dprintln!("peak found! step: {step} height: {h} dist: {dist}");
            print_heights(&heights, &pos, &start, &end, &visited);
            shortest = dist;
            break;
        }
        step += 1;
    }
    println!("shortest path from S to E: {}", shortest);

    Ok(())
}

pub fn day12_2(file_path: &str) -> io::Result<()> {
    let (heights, start, end) = read_heights(file_path);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut path: VecDeque<Path> = VecDeque::new();
    dprintln!(
        "start: ({}, {}) end: ({}, {})",
        start.0,
        start.1,
        end.0,
        end.1
    );
    let h = heights.len();
    let w = heights.first().unwrap().len();

    let mut step = 0;
    let mut shortest = 0;
    let mut pos = (0, 0);
    let mut dist = 0u32;
    path.push_back(Path { pos: end, dist: 0 });
    visited.insert(start);

    print_heights(&heights, &pos, &start, &end, &visited);
    loop {
        let this = path.pop_front();
        if this.is_some() {
            pos = this.clone().unwrap().pos;
            dist = this.clone().unwrap().dist;
        } else {
            println!("path ended!");
            break;
        }

        let mut r = (pos.0 /*+ 1*/, pos.1);
        let mut l = (pos.0 /*- 1*/, pos.1);
        let mut u = (pos.0, pos.1 /*- 1*/);
        let mut d = (pos.0, pos.1 /*+ 1*/);
        let mut right = if r.0 + 1 < w {
            r.0 += 1;
            heights[r.1][r.0]
        } else {
            100
        };
        let mut left = if l.0 >= 1 {
            l.0 -= 1;
            heights[l.1][l.0]
        } else {
            100
        };
        let mut up = if u.1 >= 1 {
            u.1 -= 1;
            heights[u.1][u.0]
        } else {
            100
        };
        let mut down = if d.1 + 1 < h {
            d.1 += 1;
            heights[d.1][d.0]
        } else {
            100
        };

        let h = heights[pos.1][pos.0];

        let eq_or_pm = |a: i32, b: i32| a - b <= 1;

        if eq_or_pm(h, right) && !visited.contains(&r) {
            visited.insert(r);
            path.push_back(Path {
                pos: r,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, left) && !visited.contains(&l) {
            visited.insert(l);
            path.push_back(Path {
                pos: l,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, up) && !visited.contains(&u) {
            visited.insert(u);
            path.push_back(Path {
                pos: u,
                dist: dist + 1,
            });
        }
        if eq_or_pm(h, down) && !visited.contains(&d) {
            visited.insert(d);
            path.push_back(Path {
                pos: d,
                dist: dist + 1,
            });
        }

        if h == 0 {
            dprintln!("a found! step: {step} height: {h} dist: {dist}");
            print_heights(&heights, &pos, &start, &end, &visited);
            shortest = dist;
            break;
        }
        step += 1;
    }
    println!("shortest path from E to a: {}", shortest);

    Ok(())
}

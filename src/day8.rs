use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::util::{dprintln, dprint};

fn get_visible<'a, I>(mut iter: I) -> Vec<bool>
where
    I: Iterator<Item = &'a i32>,
{
    let mut max_height = 0;
    let mut visible = iter
        .map(|h| {
            if h > &max_height {
                max_height = h.clone();
                return true;
            }
            false
        })
        .collect::<Vec<bool>>();
    *visible.first_mut().unwrap() = true;
    *visible.last_mut().unwrap() = true;
    visible
}

fn get_scenic_score<'a, I>(mut iter: I, height: i32) -> i32
where
    I: Iterator<Item = &'a i32>,
{
    let mut score = 0;
    for h in iter {
        score += 1;
        if *h >= height {
            break;
        }
    }
    score
}

fn read_trees(file_path: &str) -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut grid = Vec::<Vec<i32>>::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();
        let row: Vec<i32> = l
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        grid.push(row.clone());
    }
    grid
}

pub fn day8_1(file_path: &str) -> io::Result<()> {
    let mut grid = read_trees(file_path);
    let h = grid.len();
    let w = grid[0].len();
    for row in &grid {
        for col in row {
            dprint!("{col}");
        }
        dprintln!();
    }
    dprintln!("{w} x {h}");
    let mut visibility: Vec<Vec<bool>> = Vec::new();
    let mut x_visibility: Vec<Vec<bool>> = Vec::new();
    for y in 0..h {
        let mut left_vis = get_visible(grid[y].iter());
        let mut right_vis = get_visible(grid[y].iter().rev());
        right_vis.reverse();
        let vis: Vec<bool> = left_vis
            .iter()
            .zip(right_vis.iter())
            .map(|(&l, &r)| l || r)
            .collect();
        x_visibility.push(vis);
    }
    for x in 0..w {
        let mut top_vis = get_visible(grid.iter().flatten().skip(x).step_by(w));
        let mut bottom_vis = get_visible(grid.iter().rev().flatten().skip(x).step_by(w));
        bottom_vis.reverse();
        let vis: Vec<bool> = top_vis
            .iter()
            .zip(bottom_vis.iter())
            .map(|(&t, &b)| t || b)
            .collect();
        let mut vis_iter = x_visibility
            .clone()
            .into_iter()
            .flatten()
            .skip(x)
            .step_by(w);
        let combined: Vec<bool> = vis_iter
            .clone()
            .zip(vis.iter())
            .map(|(mut v, y)| {
                v = v || *y;
                v
            })
            .collect();
        visibility.push(combined);
    }
    for col in 0..w {
        for row in 0..h {
            dprint!("{}", visibility[row][col] as i32);
        }
        dprintln!();
    }

    let vis_num = visibility.iter().flatten().filter(|v| **v).count();
    println!("number of visible trees: {vis_num}");

    Ok(())
}

pub fn day8_2(file_path: &str) -> io::Result<()> {
    let mut grid = read_trees(file_path);
    let h = grid.len();
    let w = grid[0].len();

    let mut scenic_score: Vec<i32> = Vec::new();

    for (i, height) in grid.iter().flatten().enumerate() {
        let row = i / w;
        let col = i % w;
        if row == 0 || col == 0 || row == h - 1 || col == w - 1 {
            continue;
        }

        let right = grid[row].iter().skip(col + 1).take(w - (col + 1));
        let left = grid[row].iter().rev().skip(w - col).take(col);
        let up = grid
            .iter()
            .flatten()
            .rev()
            .skip(w * (h - row) + (w - col - 1))
            .step_by(w)
            .take(row);
        let down = grid
            .iter()
            .flatten()
            .skip(i + w)
            .step_by(w)
            .take(h - (row + 1));
        let this_height = grid[row][col];

        let right_score = get_scenic_score(right, this_height);
        let left_score = get_scenic_score(left, this_height);
        let up_score = get_scenic_score(up, this_height);
        let down_score = get_scenic_score(down, this_height);
        scenic_score.push(right_score * left_score * up_score * down_score);
    }

    let scenic_score_max = scenic_score.iter().max().unwrap();
    println!("maximum scenic score: {scenic_score_max}");

    Ok(())
}

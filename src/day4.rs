use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn day4_1(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let l = line?;
        let mut size = l.len();
        let pair = l
            .split(",")
            .map(|pair| pair.split("-").collect())
            .collect::<Vec<Vec<&str>>>();
        let a1 = pair[0][0].parse::<i32>().unwrap();
        let b1 = pair[0][1].parse::<i32>().unwrap();
        let a2 = pair[1][0].parse::<i32>().unwrap();
        let b2 = pair[1][1].parse::<i32>().unwrap();

        if (a1 <= a2 && b1 >= b2) || (a2 <= a1 && b2 >= b1) {
            count += 1;
        }
    }
    println!("{count}");

    Ok(())
}

pub fn day4_2(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let l = line?;
        let mut size = l.len();
        let pair = l
            .split(",")
            .map(|pair| pair.split("-").collect())
            .collect::<Vec<Vec<&str>>>();
        let a1 = pair[0][0].parse::<i32>().unwrap();
        let b1 = pair[0][1].parse::<i32>().unwrap();
        let a2 = pair[1][0].parse::<i32>().unwrap();
        let b2 = pair[1][1].parse::<i32>().unwrap();
        println!("{:?}", pair);

        if (a1 <= a2 && b1 >= b2)
            || (a2 <= a1 && b2 >= b1)
            || (a1 <= a2 && b1 >= a2)
            || (b1 >= b2 && a1 <= b2)
            || (a2 <= a1 && b2 >= a1)
            || (b2 >= b1 && a2 <= b1)
        {
            count += 1;
        }
    }
    println!("{count}");

    Ok(())
}

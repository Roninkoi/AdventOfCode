use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

use crate::util::{dprintln, dprint};

pub fn day6_1(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    for line in reader.by_ref().lines() {
        let l = line?;
        let mut lsize = l.len();
        let chars: Vec<char> = l.chars().collect();

        let mut buf = String::new();
        let buf_len = 4;
        let mut count = 0;
        for c in l.chars() {
            if buf.len() >= buf_len {
                buf.remove(0);
            }
            buf.push(c);
            dprint!(" {buf} ");
            count += 1;

            let mut dup = false;
            let mut char_map = HashSet::new();
            for bc in buf.chars() {
                let mc = char_map.get(&bc);
                if mc.is_some() || buf.len() < buf_len {
                    dup = true;
                    break;
                }
                char_map.insert(bc);
            }
            if !dup {
                break;
            }
        }
        dprintln!();
        println!("start of packet: {count}");
    }
    Ok(())
}

pub fn day6_2(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    for line in reader.by_ref().lines() {
        let l = line?;
        let mut lsize = l.len();
        let chars: Vec<char> = l.chars().collect();

        let mut buf = String::new();
        let buf_len = 14;
        let mut count = 0;
        for c in l.chars() {
            if buf.len() >= buf_len {
                buf.remove(0);
            }
            buf.push(c);
            dprint!(" {buf} ");
            count += 1;

            let mut dup = false;
            let mut char_map = HashSet::new();
            for bc in buf.chars() {
                let mc = char_map.get(&bc);
                if mc.is_some() || buf.len() < buf_len {
                    dup = true;
                    break;
                }
                char_map.insert(bc);
            }
            if !dup {
                break;
            }
        }
        dprintln!();
        println!("start of message: {count}");
    }
    Ok(())
}

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub fn day5_1(file_path: &str) -> io::Result<()> {
    crate_mover(file_path, true)
}

pub fn day5_2(file_path: &str) -> io::Result<()> {
    crate_mover(file_path, false)
}

///
/// Take a number of crates @param num from position @param from top of stack
///
fn take(stacks: &mut Vec<Vec<String>>, num: usize, from: usize) -> Vec<String> {
    let mut took = Vec::<String>::new();
    {
        let stack_from = stacks.get_mut(from - 1);
        if stack_from.is_some() {
            let sf = stack_from.unwrap();
            if sf.len() >= num {
                took = sf.split_off(sf.len() - num);
            }
        } else {
            println!("Stack {from} empty!");
        }
    }
    took
}

///
/// Place a number of crates @param took on top of stack in position @param to
///
fn place(stacks: &mut Vec<Vec<String>>, took: &mut Vec<String>, to: usize) {
    let stack_to = stacks.get_mut(to - 1);
    if stack_to.is_some() {
        let mut st = stack_to.unwrap();
        if !took.is_empty() {
            st.append(took);
        }
    }
}

fn crate_mover(file_path: &str, reverse: bool) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut stacks = Vec::<Vec<String>>::new(); // stacks of crates, bottom crate first
    let mut stack_input = Vec::<String>::new(); // stack input description
    let mut stack_grid = Vec::<Vec<String>>::new(); // parsed xy stack grid
    let mut stack_names = Vec::<String>::new(); // names of stack columns
    let mut stack_num = 0; // number of stack columns
    for line in reader.by_ref().lines() {
        let l = line?;
        let mut lsize = l.len();

        println!("{l}");

        if l.is_empty() {
            // stack description ends
            let stack_names_input = stack_input.last().unwrap();
            stack_names = stack_names_input // parse names of stacks 1-9
                .split_whitespace()
                .map(|s| s.into())
                .collect();
            let stack_names_pos: Vec<usize> = stack_input // calculate positions of stack columns
                .last()
                .unwrap()
                .split_whitespace()
                .map(|s| s.as_ptr() as usize - stack_names_input.as_ptr() as usize)
                .collect();
            stack_num = stack_names.len();
            stack_input.pop(); // throw away stack names

            stack_grid = stack_input // parse crate data
                .iter()
                .map(|row| {
                    let mut crates = Vec::<String>::new();
                    for p in &stack_names_pos {
                        let mut cs = row.chars().skip(*p); // seek for column position
                        let c = cs.next(); // look for []?
                        if c.is_some() {
                            crates.push(c.unwrap().to_string());
                        }
                    }
                    crates
                })
                .collect();
            for s in &stack_grid {
                println!("{:?}", s);
            }
            println!("stacks:\n{:?}\n{:?}", stack_names, stack_names_pos);
            break;
        }
        stack_input.push(l);
    }
    let stack_height = stack_grid.len(); // y height of stacks

    for col in 0..stack_num {
        // transpose stack grid so that we can push and pop stacks
        let mut stack_col = Vec::<String>::new();
        for row in (0..stack_height).rev() {
            let v = &stack_grid[row][col];
            if !v.is_empty() && v != " " {
                // remove empty crates
                stack_col.push(v.clone());
            }
        }
        stacks.push(stack_col);
    }

    for line in reader.by_ref().lines() {
        // parse and execute commands
        let l = line?;
        let mut words = l.split_whitespace();
        words.find(|s| *s == "move");
        let num = words.next().unwrap().parse::<usize>().unwrap(); // how many crates

        words.find(|s| *s == "from");
        let from = words.next().unwrap().parse::<usize>().unwrap(); // from where?

        words.find(|s| *s == "to");
        let to = words.next().unwrap().parse::<usize>().unwrap(); // to destination

        println!("{l} ({} {} {})", num, from, to);

        let mut took = take(&mut stacks, num, from);

        println!("took {:?}", took);
        if reverse {
            took.reverse(); // reverse order of crates (picked up one at a time)
        }

        place(&mut stacks, &mut took, to);

        for (i, s) in stacks.iter().enumerate() {
            // print state of stacks
            println!("{} {:?}", i + 1, s);
        }
    }

    for (i, s) in stacks.iter().enumerate() {
        // print top crates
        print!("{}", s.iter().last().unwrap());
    }
    println!();

    Ok(())
}

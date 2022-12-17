use regex;
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::util::{dprint, dprintln};

#[derive(Clone, Debug)]
struct Packet {
    pub input: String,
    pub value: Option<i32>,
    pub next: Option<Rc<RefCell<Packet>>>,
    pub list: Option<Rc<RefCell<Packet>>>,
}

impl Packet {
    pub fn new(input_string: Option<String>) -> Packet {
        let input = input_string.unwrap_or("".to_string());
        Packet {
            next: None,
            value: None,
            list: None,
            input: input,
        }
    }
    pub fn next(&mut self) -> Option<i32> {
        let mut value = self.value;
        if self.next.is_some() {
            value = self.value;
            //self = self.next.unwrap().borrow();
            let next = self.next.clone().unwrap();
            self.next = next.borrow().next.clone();
            self.value = next.borrow().value.clone();
            self.list = next.borrow().list.clone();
            self.input = next.borrow().input.clone();
        } else {
            if self.value.is_some() {
                self.value = None;
            }
        }
        value
    }
    pub fn list(&mut self) -> Option<i32> {
        let value = self.value;
        if self.list.is_some() {
            //self = self.next.unwrap().borrow();
            let next = self.list.clone().unwrap();
            self.next = next.borrow().next.clone();
            self.value = next.borrow().value.clone();
            self.list = next.borrow().list.clone();
            self.input = next.borrow().input.clone();
        }
        value
    }
    pub fn parse(&mut self) {
        let mut is_list = self.input.contains(['[', ']']);
        if !is_list {
            // if not a list -> parse integer
            self.value = self.input.parse::<i32>().ok();
            return;
        }

        let input = self.input.chars();
        let mut open_bracket = 0;
        let mut close_bracket = 0;
        let mut read_input = "".to_string();
        let mut reading = false; // reading list item?
        let mut current: Option<Rc<RefCell<Packet>>> = None;

        for c in input {
            match c {
                '[' => {
                    // count opening brackets
                    open_bracket += 1;
                    reading = true;
                    if open_bracket == 1 {
                        // skip first opening bracket
                        continue;
                    }
                }
                ']' => {
                    // count closing brackets
                    close_bracket += 1;
                }
                ',' => {
                    // list item -> start reading
                    if !reading {
                        reading = true;
                        continue;
                    }
                }
                ' ' => {
                    // skip whitespace
                    continue;
                }
                _ => {}
            }
            if reading {
                read_input.push(c);

                if open_bracket == close_bracket + 1 {
                    println!("list item: {read_input}");
                    let next = Rc::new(RefCell::new(Packet::new(Some(read_input.clone()))));

                    next.borrow_mut().parse(); // parse next item

                    //is_list = read_input.contains(['[', ']']);
                    is_list = false;
                    if current.is_some() {
                        let cur = current.unwrap();
                        let cur_next = cur.borrow().next.clone();
                        if cur_next.is_some() {
                            let old = cur.borrow().next.clone();
                            cur.borrow_mut().list = old;
                        }
                        if is_list {
                            cur.borrow_mut().list = Some(next.clone());
                        } else {
                            cur.borrow_mut().next = Some(next.clone());
                        }
                        current = Some(next.clone());
                    } else {
                        if is_list {
                            self.list = Some(next.clone());
                        } else {
                            self.next = Some(next.clone());
                        }
                        current = Some(next.clone());
                    }

                    reading = false;
                    read_input.clear();
                }
            }
            if open_bracket == close_bracket {
                break;
            }
        }
        println!("{:?}", self);
    }
}

fn read_packets(file_path: &str) -> Vec<Vec<Packet>> {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut pair = Vec::new();
    let mut packet_pairs = Vec::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();

        if l.is_empty() {
            if !pair.is_empty() {
                packet_pairs.push(pair.clone());
                pair.clear();
            }
            continue;
        }
        pair.push(Packet::new(Some(l)));
    }
    packet_pairs.push(pair.clone());

    for p in packet_pairs.iter_mut() {
        if p.len() < 2 {
            continue;
        }
        p[0].parse();
        p[1].parse();
    }
    packet_pairs
}

pub fn day13_1(file_path: &str) -> io::Result<()> {
    let mut packets = read_packets(file_path);

    for p in packets.iter_mut() {
        let correct = false;
        let left_is_list = !p[0].value.is_some();
        let right_is_list = !p[1].value.is_some();
        //println!("asd {left_is_list} {right_is_list} {:?}", p[0]);

        /*println!("{:?}", p[0]);
        let mut has_next = 1;
        while has_next >= 0 {
            let left = p[0].next();
            let right = p[1].next();
            if left.is_some() {
                print!(" {}< ", left.unwrap());
            }
            if right.is_some() {
                print!(" >{} ", right.unwrap());
            }
            println!();
            /*loop {
                let leftl = p[0].list();
                let rightl = p[1].list();
                if leftl.is_some() && rightl.is_some() {
                    println!("{} {}", leftl.unwrap(), rightl.unwrap());
                }
                if p[0].list.is_none() && p[0].list.is_none() && p[1].list.is_none() && p[1].list.is_none()  {
                    break;
                }
            }*/
            if p[0].next.is_some() || p[1].next.is_some()  {
                has_next += 1;
            }
            has_next -= 1;
        }*/

        /*if !left_is_list && !right_is_list { // both are integers
            let mut left: Option<Rc<RefCell<Packet>>> = Some(Rc::new(RefCell::new(p[0].clone())));
            let mut right: Option<Rc<RefCell<Packet>>> = Some(Rc::new(RefCell::new(p[1].clone())));
            println!("{} {}", left.is_some(), right.is_some());
            while left.is_some() && right.is_some() {
                let l = left.unwrap();
                let r = right.unwrap();
                println!("{} {}", l.borrow().value.unwrap(), r.borrow().value.unwrap());
                left = l.borrow().clone().next;
                right = r.borrow().clone().next;
            }
        }*/
    }

    Ok(())
}

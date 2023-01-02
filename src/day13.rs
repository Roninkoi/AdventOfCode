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
    pub fn has_next(&self) -> bool {
        self.next.is_some() || self.value.is_some()
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn has_list(&self) -> bool {
        self.list.is_some()
    }

    pub fn has_any(&self) -> bool {
        self.has_next() || self.has_list()
    }

    pub fn next(&mut self) -> Option<i32> {
        let mut value = self.value;
        if self.next.is_some() {
            value = self.value;
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
        let mut value = self.value;
        if self.list.is_some() {
            value = self.value;
            let next = self.list.clone().unwrap();
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
    pub fn add(&mut self, new: Packet) {
        let next = Rc::new(RefCell::new(self.clone()));
        self.next = Some(next);
        self.value = new.value;
        self.list = new.list;
        self.input = new.input;
    }
    pub fn parse(&mut self) {
        let mut is_list = self.input.contains(['[', ']']);
        if !is_list {
            // if not a list -> parse integer
            println!("value {}", self.input);
            self.value = self.input.parse::<i32>().ok();
            return;
        }

        let input = self.input.chars();
        let mut open_bracket = 0;
        let mut close_bracket = 0;
        let mut read_input = "".to_string();
        let mut reading = false;
        let mut closed = false;
        let mut end = false;
        let mut current: Option<Rc<RefCell<Packet>>> = None;

        for c in input {
            match c {
                '[' => {
                    open_bracket += 1;
                    // count opening brackets
                    reading = true;
                    if open_bracket == 1 {
                        // skip first opening bracket
                        continue;
                    }
                }
                ']' => {
                    // count closing brackets
                    close_bracket += 1;

                    if open_bracket == close_bracket { // last bracket
                        end = true;
                        closed = true;
                        reading = false;
                    }
                }
                ',' => {
                    if !reading {
                        reading = true;
                    }
                    if open_bracket == close_bracket + 1 {
                        closed = true;
                        reading = false;
                    }
                }
                ' ' => {
                    // skip whitespace
                    continue;
                }
                _ => { }
            }
            if reading {
                read_input.push(c);
            }
            if closed {
                dprintln!("list item: {read_input}");
                let next = Rc::new(RefCell::new(Packet::new(Some(read_input.clone()))));

                next.borrow_mut().parse(); // parse next item

                if current.is_some() {
                    let mut cur = current.unwrap();
                    cur.borrow_mut().next = Some(next.clone());
                    current = Some(next.clone());
                } else {
                    self.list = Some(next.clone());
                    current = Some(next.clone());
                }

                reading = true;
                closed = false;
                read_input.clear();
            }

            if end {
                break;
            }
        }
        dprintln!("{:?}", self);
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

fn print_packet(left: Rc<RefCell<Packet>>) {
    loop {
        if left.borrow().has_list() {
            let left_list = Rc::new(RefCell::new(left.borrow().clone()));
            left_list.borrow_mut().list();
            print!("\n[");
            print_packet(left_list);
            println!("]");
        }
        if !left.borrow().has_next() {
            break;
        }
        let left_value = left.borrow_mut().next();
        if left_value.is_some() {
            print!("{} ", left_value.unwrap());
        }
    }
}

fn packets_ordered(left: Rc<RefCell<Packet>>, right: Rc<RefCell<Packet>>) -> Option<bool> {
    loop {
        println!("left: {:?}", left.clone());
        println!("right: {:?}", right.clone());
        if left.borrow().has_list() || right.borrow().has_list() {
            let left_list = Rc::new(RefCell::new(left.borrow_mut().clone()));
            let right_list = Rc::new(RefCell::new(right.borrow_mut().clone()));
            if left.borrow().has_list() {
                left_list.borrow_mut().list();
            } else if left.borrow().has_value() {
                let new = left_list.borrow().clone();
                //left_list.borrow_mut().add(new.clone());
                left_list.borrow_mut().next = None;
                left_list.borrow_mut().list = None;
            } else {
                dprintln!("true, left list ran out!");
                return Some(true);
            }
            if right.borrow().has_list() {
                right_list.borrow_mut().list();
            } else if right.borrow().has_value() {
                let new = right_list.borrow().clone();
                //right_list.borrow_mut().add(new.clone());
                right_list.borrow_mut().next = None;
                left_list.borrow_mut().list = None;
            } else {
                dprintln!("false, right list ran out!");
                return Some(false);
            }
            let ordered = packets_ordered(left_list, right_list);
            if ordered.is_some() {
                return ordered;
            }
        }
        if !right.borrow().has_any() && left.borrow().has_any() {
            dprintln!("false, right ran out!");
            return Some(false);
        }
        else if !left.borrow().has_any() && right.borrow().has_any() {
            dprintln!("true, left ran out!");
            return Some(true);
        }
        else if !right.borrow().has_any() && !left.borrow().has_any() {
            return None;
        }
        let left_value = left.borrow_mut().next();
        let right_value = right.borrow_mut().next();
        if left_value.is_some() && right_value.is_some() {
            let l = left_value.unwrap();
            let r = right_value.unwrap();
            dprint!(" {}< ", l);
            dprint!(" >{} ", r);
            dprintln!();
            if l > r {
                dprintln!("false, left larger!");
                return Some(false);
            }
            if l < r {
                dprintln!("true, right larger!");
                return Some(true);
            }
        }
    }
}

pub fn day13_1(file_path: &str) -> io::Result<()> {
    let mut packets = read_packets(file_path);

    let mut correct = 0;
    let mut index_sum = 0;
    for (i, p) in packets.iter_mut().enumerate() {
        let ordered = packets_ordered(
            Rc::new(RefCell::new(p[0].clone())),
            Rc::new(RefCell::new(p[1].clone())),
        ).unwrap();
        if ordered {
            index_sum += i + 1;
        }
        dprintln!("ordered: {ordered}");
        /*println!("left");
        print_packet(Rc::new(RefCell::new(p[0].clone())));
        println!("right");
        print_packet(Rc::new(RefCell::new(p[1].clone())));*/
    }
    println!("sum of indices in correct order: {index_sum}");

    Ok(())
}

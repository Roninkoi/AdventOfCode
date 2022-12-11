use regex;
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Monke {
    pub id: String,                   // monkey id
    pub items: Vec<u64>,              // list of item worry levels
    pub op: (String, String, String), // (op, arg1, arg2)
    pub test: u64,                    // divisible by test
    pub dest: (String, String),       // destination (true, false)
    pub activity: u64,                // number of items inspected
}

impl Monke {
    pub fn new() -> Monke {
        Monke {
            id: "0".to_string(),
            items: Vec::new(),
            op: ("".to_string(), "".to_string(), "".to_string()),
            test: 1,
            dest: ("0".to_string(), "0".to_string()),
            activity: 0,
        }
    }
}

fn read_monke(file_path: &str) -> Vec<Monke> {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut monke = Monke::new();
    let mut monkeys = Vec::<Monke>::new();
    let mut dest = false;
    let mut dest_i = 0;
    for line in reader.by_ref().lines() {
        let l = line.unwrap();

        if l.is_empty() {
            monkeys.push(monke.clone());
            monke = Monke::new();
            continue;
        }
        let re = regex::Regex::new(r"\s|:|,").unwrap();
        let words: Vec<&str> = re.split(&l).collect();
        let mut first = words[0];
        if first.is_empty() {
            first = words[2];
        }
        if dest {
            let monke_id = words[10].to_string();

            if dest_i == 0 {
                monke.dest.0 = monke_id;
            } else if dest_i >= 1 {
                monke.dest.1 = monke_id;
                dest = false;
            }
            dest_i += 1;
        }

        match first {
            "Monkey" => {
                monke.id = words[1].to_string();
            }
            "Starting" => {
                // items
                for i in (5..words.len()).step_by(2) {
                    let item = words[i].parse::<u64>().unwrap();
                    monke.items.push(item);
                }
            }
            "Operation" => {
                let op = words[7].to_string();
                let arg1 = words[6].to_string();
                let arg2 = words[8].to_string();
                monke.op = (op, arg1, arg2);
            }
            "Test" => {
                let test = words[6].parse::<u64>().unwrap();
                monke.test = test;
                dest = true;
                dest_i = 0;
            }
            _ => {}
        }
    }
    monkeys.push(monke.clone());
    monkeys
}

fn do_rounds(file_path: &str, n_rounds: u32, worry_div: u64) -> Vec<Monke> {
    let mut monkeys = read_monke(file_path);
    let test_mod = monkeys.iter().map(|m| m.test).product::<u64>();

    let get_arg = |a: &String, old: &u64| match a.as_str() {
        "old" => return *old,
        _ => return a.parse::<u64>().unwrap(),
    };

    let inspect = |op: &String, arg1: &String, arg2: &String, old: &u64| {
        let a = get_arg(arg1, old);
        let b = get_arg(arg2, old);

        match op.as_str() {
            "+" => {
                return a + b;
            }
            "-" => {
                return a - b;
            }
            "*" => {
                return a * b;
            }
            "/" => {
                return a / b;
            }
            _ => {
                return a;
            }
        }
    };

    for _ in 0..n_rounds {
        //println!("round {round}");
        for i in 0..monkeys.len() {
            let monke = monkeys[i].clone();
            //println!("{:?}", monke);
            let (op, arg1, arg2) = monke.op;

            for (j, item) in monke.items.iter().enumerate() {
                let mut worry: u64 = (inspect(&op, &arg1, &arg2, item) / worry_div) % test_mod;
                //println!("{item} -> {worry}");
                if worry % monke.test == 0 {
                    monkeys
                        .iter_mut()
                        .filter(|m| m.id == monke.dest.0)
                        .next()
                        .unwrap()
                        .items
                        .push(worry);
                    //println!("throwing to monkey {}", monke.dest.0);
                } else {
                    monkeys
                        .iter_mut()
                        .filter(|m| m.id == monke.dest.1)
                        .next()
                        .unwrap()
                        .items
                        .push(worry);
                    //println!("throwing to monkey {}", monke.dest.1);
                }
                monkeys[i].items[j] = 0;
                monkeys[i].activity += 1;
            }
            monkeys[i].items.retain(|item| *item > 0);
        }
    }

    return monkeys;
}

pub fn day11_1(file_path: &str) -> io::Result<()> {
    let mut monkeys = do_rounds(file_path, 20, 3);
    for monke in monkeys.clone() {
        println!("{:?}", monke);
    }

    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));
    let monkey_business: u64 = monkeys.iter().take(2).map(|m| m.activity).product();
    println!("level of monkey business: {monkey_business}");

    Ok(())
}

pub fn day11_2(file_path: &str) -> io::Result<()> {
    let mut monkeys = do_rounds(file_path, 10_000, 1);
    for monke in monkeys.clone() {
        println!("{:?}", monke);
    }

    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));
    let monkey_business: u64 = monkeys.iter().take(2).map(|m| m.activity).product();
    println!("level of monkey business: {monkey_business}");

    Ok(())
}

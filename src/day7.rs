use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::util::{dprint, dprintln};

#[derive(Clone, Debug, PartialEq)]
struct DirTree {
    pub parent: Option<Rc<RefCell<DirTree>>>,
    pub child: Vec<Rc<RefCell<DirTree>>>,
    pub dir_name: String,
    pub file: HashMap<String, i32>,
    pub size: i32,
}

impl DirTree {
    pub fn new() -> DirTree {
        DirTree {
            parent: None,
            child: Vec::new(),
            dir_name: String::new(),
            file: HashMap::new(),
            size: 0,
        }
    }
    pub fn find(&mut self, dir_name: &String) -> Option<Rc<RefCell<DirTree>>> {
        let mut existing = None;
        for c in &self.child {
            let child = c.borrow().clone();
            if child.dir_name == *dir_name {
                existing = Some(c.clone());
                break;
            }
        }
        existing
    }
    pub fn add_dir(&mut self, dir_name: String, parent: Option<Rc<RefCell<DirTree>>>) {
        let mut existing = self.find(&dir_name);
        if existing.is_none() {
            let mut child = Rc::new(RefCell::new(DirTree::new()));
            child.borrow_mut().dir_name = dir_name;
            child.borrow_mut().parent = parent;
            self.child.push(child);
        }
    }
    pub fn add_file(&mut self, file_name: String, file_size: i32) {
        self.file.insert(file_name, file_size);
    }
    pub fn calc_size(&mut self) -> i32 {
        let mut size_tot = self.file.values().sum::<i32>();
        for c in &self.child {
            let dir_size = c.borrow_mut().calc_size();
            size_tot += dir_size;
        }
        self.size = size_tot;
        size_tot
    }
    pub fn sum_leq(&mut self, leq: i32) -> i32 {
        let mut size_tot = 0;
        if self.size <= leq {
            dprintln!("leq {} {}", self.dir_name, self.size);
            size_tot += self.size;
        }
        for c in &self.child {
            let dir_size = c.borrow_mut().sum_leq(leq);
            size_tot += dir_size;
        }
        size_tot
    }
    pub fn smallest_geq(&mut self, smallest: i32, geq: i32) -> i32 {
        let mut best = smallest;
        if self.size >= geq && self.size <= best {
            dprintln!("found smallest {} {}", self.dir_name, self.size);
            best = self.size;
        }
        for c in &self.child {
            let dir_size = c.borrow_mut().smallest_geq(best, geq);

            if dir_size >= geq && dir_size <= best {
                best = dir_size;
            }
        }
        best
    }
}

#[derive(Clone, Debug)]
struct FileSystem {
    root: Rc<RefCell<DirTree>>,
    current: Rc<RefCell<DirTree>>,
    parent: Option<Rc<RefCell<DirTree>>>,
    size: i32,
    capacity: i32,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let mut directories = DirTree::new();
        directories.dir_name = "/".to_string();
        let current = Rc::new(RefCell::new(directories));
        FileSystem {
            root: current.clone(),
            current: current.clone(),
            parent: None,
            size: 0,
            capacity: 70000000,
        }
    }
    pub fn root(&mut self) {
        self.current = self.root.clone();

        for dir_child in &self.current.borrow().child {
            let child = &dir_child.borrow().dir_name;
            dprint!("{} ", child)
        }
        dprintln!(
            "{:?}",
            self.current.borrow().file.keys().collect::<Vec<&String>>()
        );
    }
    pub fn up(&mut self) {
        if self.parent.is_some() {
            let mut parent = self.parent.clone().unwrap();
            self.parent = parent.borrow().parent.clone();
            self.current = parent.clone();

            for dir_child in &self.current.borrow().child {
                let child = &dir_child.borrow().dir_name;
                dprint!("{} ", child)
            }
            dprintln!(
                "{:?}",
                self.current.borrow().file.keys().collect::<Vec<&String>>()
            );
        } else {
            println!("can't go up from {}", self.current.borrow().dir_name);
        }
    }
    pub fn down(&mut self, dir_name: String) {
        if dir_name == "/" {
            self.root();
            return;
        }
        let mut existing = self.current.borrow_mut().find(&dir_name);
        if existing.is_some() {
            let child = existing.clone().unwrap();
            self.parent = child.borrow().parent.clone();
            self.current = child.clone();

            for dir_child in &self.current.borrow().child {
                let child = &dir_child.borrow().dir_name;
                dprint!("{} ", child)
            }
            dprintln!(
                "{:?}",
                self.current.borrow().file.keys().collect::<Vec<&String>>()
            );
        } else {
            println!(
                "dir {dir_name} not found in {}",
                self.current.borrow().dir_name
            );
        }
    }
    pub fn add_dir(&mut self, dir_name: String) {
        self.current
            .borrow_mut()
            .add_dir(dir_name, Some(self.current.clone()));
    }
    pub fn add_file(&mut self, file_name: String, file_size: i32) {
        self.current.borrow_mut().add_file(file_name, file_size);
    }
    pub fn calc_size(&mut self) -> i32 {
        let size = self.root.borrow_mut().calc_size();
        self.size = size;
        size
    }
    pub fn sum_leq(&mut self, leq: i32) -> i32 {
        self.root.borrow_mut().sum_leq(leq)
    }
    pub fn find_remove(&mut self, size: i32) -> i32 {
        let req_capacity = size - (self.capacity - self.size);
        if req_capacity <= 0 {
            return 0;
        }
        self.root
            .borrow_mut()
            .smallest_geq(self.capacity, req_capacity)
    }
}

fn read_file_system(file_path: &str) -> FileSystem {
    let stdin = io::stdin();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut listing = false;
    let mut command: String = "".to_string();
    let mut dir: String = "/".to_string();
    let mut file_system = FileSystem::new();
    for line in reader.by_ref().lines() {
        let l = line.unwrap();
        let words: Vec<&str> = l.split_whitespace().collect();

        if words.len() < 1 {
            continue;
        }
        if words[0] == "$" {
            listing = false;
            let cmd = words[1];
            match cmd {
                "ls" => {
                    listing = true;
                }
                "cd" => {
                    dir = words[2].to_string();
                    if dir == ".." {
                        file_system.up();
                    } else {
                        file_system.down(dir.to_owned());
                    }
                }
                _ => {}
            }
            command = cmd.to_string();
            dprintln!("$ {command} {dir}");
        } else if listing {
            let file_name = words[1].to_string();
            if words[0] == "dir" {
                file_system.add_dir(file_name.to_owned());
                dprintln!("added dir {file_name}");
            } else {
                let file_size = words[0].parse::<i32>().unwrap();
                file_system.add_file(file_name.to_owned(), file_size);
                dprintln!("added file {file_name} with size {file_size}");
            }
        }
    }
    return file_system;
}

pub fn day7_1(file_path: &str) -> io::Result<()> {
    let mut file_system = read_file_system(file_path);
    dprint!("root: ");
    file_system.root();
    let size_tot = file_system.calc_size();
    dprintln!("total filesystem size: {size_tot}");
    println!("less or equal to 100000: {}", file_system.sum_leq(100000));

    Ok(())
}

pub fn day7_2(file_path: &str) -> io::Result<()> {
    let mut file_system = read_file_system(file_path);
    dprint!("root: ");
    file_system.root();
    let size_tot = file_system.calc_size();
    dprintln!("total filesystem size: {size_tot}");

    println!("dir to remove: {}", file_system.find_remove(30000000));

    Ok(())
}

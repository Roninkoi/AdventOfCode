use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/**
 * Calculate priorities of items in pocket: a-z = 1-26, A-Z = 27-52
 */
fn get_priority(pocket: &str) -> Vec<u32> {
    pocket
        .chars()
        .into_iter()
        .map(|item| {
            let p = item.to_lowercase().next().unwrap() as i32 - ('a' as u8 - 1) as i32
                + (item.is_uppercase() as i32) * 26;
            p.abs() as u32
        })
        .collect::<Vec<u32>>()
}

pub fn day3_1(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut prio_tot = 0;
    let mut n = 0;
    for line in reader.lines() {
        let l = line?;
        let mut size = l.len();
        if size % 2 == 1 {
            continue;
        }
        let pockets = [&l[0..size / 2], &l[size / 2..size]];
        let prios = [get_priority(pockets[0]), get_priority(pockets[1])];

        let mut pocket1 = HashSet::new();
        let mut pocket2 = HashSet::new();
        let mut prios1 = HashMap::new();
        let mut prios2 = HashMap::new();

        for i in 0..size / 2 {
            let item1 = pockets[0].chars().nth(i).unwrap();
            let item2 = pockets[1].chars().nth(i).unwrap();
            pocket1.insert(item1);
            pocket2.insert(item2);

            let p1 = prios1.get(&item1);
            let p2 = prios2.get(&item2);
            let mut prio1 = prios[0][i];
            let mut prio2 = prios[1][i];
            // if multiple items of same type in pocket, do we add up priorities?
            if p1 != None {
                //prio1 += p1.unwrap();
            }
            if p2 != None {
                //prio2 += p2.unwrap();
            }
            prios1.insert(item1, prio1);
            prios2.insert(item2, prio2);
        }
        let intersect = pocket1.intersection(&pocket2);

        println!("pockets in bag {n}:");
        for p in &prios[0] {
            print!("{p} ");
        }
        println!("= {}", pockets[0]);
        for p in &prios[1] {
            print!("{p} ");
        }
        println!("= {}", pockets[1]);

        let mut prio = 0;
        println!("intersection of bag {n}:");
        for isect in intersect {
            print!("{isect} ");
            prio += prios1.get(isect).unwrap();
            // do we add items in both pockets?
            //prio += prios2.get(isect).unwrap();
        }
        println!("priority: {prio}");
        prio_tot += prio;
        n += 1;
    }
    println!("total priority in {n} bags: {prio_tot}");

    Ok(())
}

pub fn day3_2(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut bags: Vec<Vec<String>> = Vec::new(); // bags, each containing n_pockets pockets
    let mut pockets = Vec::new(); // pockets, each containing any number of items
    let n_pockets = 3; // number of pockets in bag (2 for part 1, 3 for part 2)
    let multiline = true; // read pockets over multiple lines (false for part 1, true for part 2)
    let mut n_bags = 0;
    let mut i_line = 0;
    for line in reader.lines() { // read bags and pockets
        let l = line?;
        let mut size = l.len();

        println!("bag {n_bags}: {l}");
        if multiline {
            let mut pocket = l.to_string();
            pockets.push(pocket);
            i_line += 1;
            if i_line % n_pockets == 0 { // last pocket, move to next bag
                bags.push(pockets.clone());
                pockets.clear();
                i_line = 0;
            } else {
                continue;
            }
        } else {
            if size % n_pockets != 0 {
                continue;
            }
            let pocket_size = size / n_pockets;
            for i in 0..n_pockets { // read equal-sized pockets from line
                let mut pocket = l[i * pocket_size..(i + 1) * pocket_size].to_string();
                println!("pocket: {}", pocket);
                pockets.push(pocket);
            }
            bags.push(pockets.clone());
            pockets.clear();
        }

        n_bags += 1;
    }

    // compute priorities of items in all pockets in all bags
    let prios = bags
        .iter()
        .map(|bag| {
            bag.iter()
                .map(|pocket| get_priority(pocket))
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut prio_tot = 0;
    for bag in 0..bags.len() {
        let mut pocket_set_prev = None;
        println!("bag {bag}");
        let mut isect: String = String::new(); // intersection between all pockets
        for pocket in 0..bags[bag].len() {
            let pocket_set = HashSet::<char>::from_iter(bags[bag][pocket].chars());
            println!("pocket {pocket} set {}", pocket_set.clone().into_iter().collect::<String>());

            // calculate intersection between this pocket and previous pocket
            pocket_set_prev = if pocket_set_prev.is_some() {
                let prev = pocket_set_prev.unwrap();
                let intersect: HashSet<char> = (&pocket_set) & (&prev);
                Some(intersect)
            } else {
                Some(pocket_set)
            };
            isect = pocket_set_prev.clone().unwrap().into_iter().collect();
            println!("isect {isect}");

            print!("pocket {pocket} {} = ", bags[bag][pocket]);
            for prio in &prios[bag][pocket] {
                print!("{prio} ");
            }
            println!();
        }
        // add up priorities of intersection (though in AoC input there is only one element)
        let isect_prio: u32 = get_priority(isect.as_str()).iter().sum();
        println!("isect prio {isect_prio}");
        prio_tot += isect_prio;
    }

    println!("total priority in {n_bags} bags: {prio_tot}");

    Ok(())
}

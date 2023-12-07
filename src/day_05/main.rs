use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Entry {
    dst: u128,
    src: u128,
    range: u128,
}

impl Entry {
    pub fn new(dst: u128, src: u128, range: u128) -> Self {
        Entry { dst, src, range }
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.src.partial_cmp(&other.src)
    }
}
#[derive(Debug)]
struct Seed {
    start: u128,
    range: u128,
}

impl Seed {
    pub fn new(start: u128, range: u128) -> Self {
        Seed { start, range }
    }
}

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_05/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut contents: Vec<&str> = contents.lines().collect();

    let mut almanac: Vec<Vec<Entry>> = vec![];

    // Get the seeds
    let l = contents.remove(0);
    let seeds: Vec<&str> = l.split(':').collect();
    let seeds = seeds[1];
    let seeds: Vec<u128> = seeds
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let temp = seeds;
    let mut seeds: Vec<Seed> = vec![];

    let mut prev_n: Option<u128> = None;
    for n in temp {
        match prev_n {
            Some(prev_number) => {
                seeds.push(Seed::new(prev_number, n));
                prev_n = None;
            }
            None => prev_n = Some(n),
        }
    }

    dbg!(&seeds);

    // Get the conversions
    let mut new_line_before = true;

    for line in contents {
        if !line.chars().any(|c| c.is_digit(10)) {
            new_line_before = true;
            continue;
        }
        if line.len() == 0 {
            new_line_before = true;
        }
        if new_line_before {
            new_line_before = false;
            almanac.push(vec![]);
        }
        match almanac.last_mut() {
            Some(v) => {
                let numbers: Vec<u128> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                if numbers.len() != 3 {
                    continue;
                }
                let dst = numbers[0];
                let src = numbers[1];
                let range = numbers[2];

                v.push(Entry::new(dst, src, range));
            }
            None => {}
        }
    }

    for mut v in almanac.iter_mut() {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    // dbg!(&almanac);

    let mut smallest_location: Option<u128> = None;

    for mut s in seeds {
        println!("New seed");
        for mut seed in s.start..s.start + s.range {
            for v in &almanac {
                for e in v.iter() {
                    if e.src <= seed && e.src + e.range > seed {
                        seed = (seed - e.src) + e.dst;
                        break;
                    }
                }
            }
            match smallest_location {
                Some(l) => {
                    if l > seed {
                        smallest_location = Some(seed)
                    }
                }
                None => smallest_location = Some(seed),
            }
        }
    }

    dbg!(&smallest_location);

    Ok(())
}
// use std::cmp::Ordering;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{self, Read};

// #[derive(Debug, PartialEq)]
// struct Entry {
//     dst: u128,
//     src: u128,
//     range: u128,
// }

// impl Entry {
//     pub fn new(dst: u128, src: u128, range: u128) -> Self {
//         Entry { dst, src, range }
//     }
// }
// impl PartialOrd for Entry {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.src.partial_cmp(&other.src)
//     }
// }

// fn main() -> io::Result<()> {
//     // Open a file in read-only mode
//     let file_path = "src/day_05/input";
//     let mut file = File::open(file_path)?;

//     // Read the contents of the file into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let mut contents: Vec<&str> = contents.lines().collect();

//     let mut almanac: Vec<Vec<Entry>> = vec![];

//     // Get the seeds
//     let l = contents.remove(0);
//     let seeds: Vec<&str> = l.split(':').collect();
//     let seeds = seeds[1];
//     let seeds: Vec<u128> = seeds
//         .split_whitespace()
//         .map(|n| n.parse().unwrap())
//         .collect();

//     // Get the conversions
//     let mut new_line_before = true;

//     for line in contents {
//         if !line.chars().any(|c| c.is_digit(10)) {
//             new_line_before = true;
//             continue;
//         }
//         if line.len() == 0 {
//             new_line_before = true;
//         }
//         if new_line_before {
//             new_line_before = false;
//             almanac.push(vec![]);
//         }
//         match almanac.last_mut() {
//             Some(v) => {
//                 let numbers: Vec<u128> = line
//                     .split_whitespace()
//                     .map(|s| s.parse().unwrap())
//                     .collect();
//                 if numbers.len() != 3 {
//                     continue;
//                 }
//                 let dst = numbers[0];
//                 let src = numbers[1];
//                 let range = numbers[2];

//                 v.push(Entry::new(dst, src, range));
//             }
//             None => {}
//         }
//     }

//     for mut v in almanac.iter_mut() {
//         v.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     }

//     dbg!(&almanac);

//     let mut locations: Vec<u128> = vec![];

//     for mut seed in seeds {
//         for v in &almanac {
//             for e in v.iter() {
//                 if e.src <= seed && e.src + e.range > seed {
//                     seed = (seed - e.src) + e.dst;
//                     break;
//                 }
//             }
//         }
//         locations.push(seed);
//     }

//     locations.sort();
//     dbg!(&locations);

//     Ok(())
// }

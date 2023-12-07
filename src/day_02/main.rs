use std::fs::File;
use std::io::{self, Read};
use std::str::Lines;

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_02/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cube_names = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut sum = 0;

    let lines = contents.lines();

    for (i, mut line) in lines.enumerate() {
        let mut max_cubes = vec![0, 0, 0]; // red | green | blue
        let mut possible = true;
        // Cut of game
        if let Some(rest) = line.get(5..) {
            line = rest;
        } else {
            continue;
        }

        // Get number of game
        let mut game_nbr: usize = 0;

        if let Some(index) = line.find(":") {
            let game_number_str = line.get(..index).unwrap();
            game_nbr = game_number_str.parse::<usize>().unwrap();
            line = line.get(index + 1..).unwrap();
        } else {
            continue;
        }

        // split on ;
        for draw in line.split(';') {
            // split on ,
            for part in draw.split(',') {
                let components: Vec<&str> = part.trim().split(' ').collect();
                if components.len() != 2 {
                    continue;
                }
                if let Some(index) = cube_names.iter().position(|name| name == components[1]) {
                    // here i know the index of what word and the parsed value
                    if let Ok(nbr) = components[0].parse::<i32>() {
                        if max_cubes[index] < nbr {
                            max_cubes[index] = nbr;
                        }
                    }
                } else {
                    continue;
                }
            }
        }

        sum = sum + max_cubes[0] * max_cubes[1] * max_cubes[2];
    }

    println!("Summed up numbers: {}", sum);
    Ok(())
}
// use std::fs::File;
// use std::io::{self, Read};
// use std::str::Lines;

// fn main() -> io::Result<()> {
//     // Open a file in read-only mode
//     let file_path = "src/day_02/input";
//     let mut file = File::open(file_path)?;

//     // Read the contents of the file into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let mut combined_possible_ids = 0;

//     let max_cubes = vec![12, 13, 14]; // red | green | blue
//     let cube_names = vec![
//         String::from("red"),
//         String::from("green"),
//         String::from("blue"),
//     ];

//     let lines = contents.lines();

//     for (i, mut line) in lines.enumerate() {
//         let mut possible = true;
//         // Cut of game
//         if let Some(rest) = line.get(5..) {
//             line = rest;
//         } else {
//             continue;
//         }

//         // Get number of game
//         let mut game_nbr: usize = 0;

//         if let Some(index) = line.find(":") {
//             let game_number_str = line.get(..index).unwrap();
//             game_nbr = game_number_str.parse::<usize>().unwrap();
//             line = line.get(index + 1..).unwrap();
//         } else {
//             continue;
//         }

//         // split on ;
//         for draw in line.split(';') {
//             // split on ,
//             for part in draw.split(',') {
//                 let components: Vec<&str> = part.trim().split(' ').collect();
//                 if components.len() != 2 {
//                     possible = false;
//                     continue;
//                 }
//                 if let Some(index) = cube_names.iter().position(|name| name == components[1]) {
//                     if max_cubes[index] < components[0].parse::<i32>().unwrap() {
//                         possible = false;
//                         continue;
//                     }
//                 } else {
//                     possible = false;
//                     continue;
//                 }
//             }
//         }
//         if possible {
//             combined_possible_ids = combined_possible_ids + game_nbr;
//         }
//     }

//     println!("Summed up numbers: {}", combined_possible_ids);
//     Ok(())
// }

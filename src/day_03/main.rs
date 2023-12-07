use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_03/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let engine: Vec<&str> = contents.lines().collect();

    let mut result: i128 = 0;

    dbg!(&engine);

    let schematics: Vec<Vec<char>> = engine.iter().map(|str| str.chars().collect()).collect();

    for (y, line) in schematics.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                let c_gears = check_connected_numbers(x, y, &schematics);
                dbg!(&c_gears);
                if c_gears.len() == 2 {
                    result = result + c_gears[0] as i128 * c_gears[1] as i128;
                }
            }
        }
        println!("");
    }

    dbg!(result);
    Ok(())
}

// function for
fn check_connected_numbers(x: usize, y: usize, schematics: &Vec<Vec<char>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut x_min: usize = 0;
    if x != 0 {
        x_min = x - 1;
    }

    let mut x_max: usize = x + 1;
    if x + 1 >= schematics[0].len() {
        x_max = x;
    }

    let mut y_min: usize = 0;
    if y != 0 {
        y_min = y - 1;
    }

    let mut y_max: usize = y + 1;
    if y + 1 >= schematics.len() {
        y_max = schematics.len() - 1;
    }

    let mut last_was_number = false;
    println!("-----");
    for j in y_min..=y_max {
        for mut i in x_min..=x_max {
            let c = schematics[j][i];
            print!("{}", c);
            if last_was_number && c.is_numeric() {
                continue;
            }
            last_was_number = false;
            if c.is_numeric() {
                last_was_number = true;
                result.push(get_number(i, j, schematics));
            }
        }
        println!("");
        last_was_number = false;
    }

    return result;
}

fn get_number(mut x: usize, y: usize, schematics: &Vec<Vec<char>>) -> i32 {
    let mut result: i32 = 0;
    while x > 0 && schematics[y][x - 1].is_numeric() {
        x = x - 1;
    }

    while x < schematics[y].len() && schematics[y][x].is_numeric() {
        result = result * 10 + schematics[y][x].to_digit(10).unwrap() as i32;
        x = x + 1;
    }

    return result;
}

// use std::fs::File;
// use std::io::{self, Read};

// fn main() -> io::Result<()> {
//     // Open a file in read-only mode
//     let file_path = "src/day_03/input";
//     let mut file = File::open(file_path)?;

//     // Read the contents of the file into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let engine: Vec<&str> = contents.lines().collect();

//     let mut result = 0;

//     dbg!(&engine);

//     let mut current_number: i32 = -1;
//     let mut adjacent_to_symbol = false;

//     let schematics: Vec<Vec<char>> = engine.iter().map(|str| str.chars().collect()).collect();

//     for (y, line) in schematics.iter().enumerate() {
//         current_number = -1;
//         adjacent_to_symbol = false;

//         for (x, c) in line.iter().enumerate() {
//             if !c.is_numeric() {
//                 if adjacent_to_symbol {
//                     result = result + current_number;
//                     // println!("RES {} {}", result, current_number);
//                 }

//                 current_number = -1;
//                 adjacent_to_symbol = false;
//                 continue;
//             }
//             if !adjacent_to_symbol {
//                 adjacent_to_symbol = check_if_symbol_adjacent(x, y, &schematics);
//             }

//             if current_number == -1 {
//                 current_number = c.to_digit(10).unwrap() as i32;
//             } else {
//                 current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
//             }
//             // println!("{} {} {} {}", x, y, current_number, adjacent_to_symbol);
//         }
//         if adjacent_to_symbol {
//             result = result + current_number;
//         }
//     }

//     dbg!(result);
//     Ok(())
// }

// // function for
// fn check_if_symbol_adjacent(x: usize, y: usize, schematics: &Vec<Vec<char>>) -> bool {
//     let mut x_min: usize = 0;
//     if x != 0 {
//         x_min = x - 1;
//     }

//     let mut x_max: usize = x + 1;
//     if x + 1 >= schematics[0].len() {
//         x_max = x;
//     }

//     let mut y_min: usize = 0;
//     if y != 0 {
//         y_min = y - 1;
//     }

//     let mut y_max: usize = y + 1;
//     if y + 1 >= schematics.len() {
//         y_max = schematics.len() - 1;
//     }

//     println!("-----");
//     for j in y_min..=y_max {
//         for i in x_min..=x_max {
//             let c = schematics[j][i];
//             print!("{}", c);
//             if (c < '0' || c > '9') && c != '.' {
//                 println!("");
//                 return true;
//             }
//         }
//         println!("");
//     }

//     return false;
// }

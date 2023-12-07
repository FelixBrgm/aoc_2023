use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_06/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut contents: Vec<&str> = contents.lines().collect();

    dbg!(&contents);
    // 37286485
    // 37286485
    let times: &str = contents.remove(0);
    let times: Vec<u128> = times
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let distances = contents.remove(0);
    let distances: Vec<u128> = distances
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let mut score: u128 = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let mut number_of_ways_to_win: u128 = 0;
        for time_held in 1..time {
            let time_to_travel = time - time_held;
            if time_to_travel * time_held > distance {
                number_of_ways_to_win = number_of_ways_to_win + 1;
            }
        }
        if number_of_ways_to_win > 0 {
            score = score * number_of_ways_to_win;
        }
    }

    dbg!(&score);
    Ok(())
}

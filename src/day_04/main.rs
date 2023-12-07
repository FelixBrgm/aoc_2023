use std::fs::File;
use std::io::{self, Read};
use std::u32;

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_04/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let scratchcards: Vec<&str> = contents.lines().collect();

    dbg!(&scratchcards);

    let mut score = 0;

    let mut card_score;

    let mut multiplications: Vec<u32> = vec![];

    let mut game_id: u32 = 0;

    for card in scratchcards {
        game_id = game_id + 1;
        card_score = 0;

        let numbers: Vec<&str> = card.split(':').collect();
        let numbers: &str = numbers[1];
        let numbers: Vec<&str> = numbers.trim().split('|').collect();

        let winning_numbers: &str = numbers[0];
        let ticket_numbers: &str = numbers[1];

        let winning_numbers: Vec<u8> = winning_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let ticket_numbers: Vec<u8> = ticket_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        for n in ticket_numbers {
            if let Some(_) = winning_numbers.iter().find(|&w_n| *w_n == n) {
                card_score = card_score + 1;
            }
        }

        // i know the card score
        let mut multiplication = 1;
        if !multiplications.is_empty() {
            multiplication = multiplications.remove(0);
        }

        println!(
            "ID: Score: {} | Multiplications: {}",
            card_score, multiplication
        );
        score = score + multiplication;

        for _ in 0..multiplication {
            for i in 0..card_score {
                if multiplications.len() <= i as usize {
                    multiplications.push(1);
                }
                multiplications[i as usize] = multiplications[i as usize] + 1;
            }
        }
    }

    dbg!(&score);
    Ok(())
}
// use core::num;
// use std::fs::File;
// use std::io::{self, Read};

// fn main() -> io::Result<()> {
//     // Open a file in read-only mode
//     let file_path = "src/day_04/input";
//     let mut file = File::open(file_path)?;

//     // Read the contents of the file into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let scratchcards: Vec<&str> = contents.lines().collect();

//     let mut score = 0;

//     let mut card_score;

//     for card in scratchcards {
//         card_score = 0;

//         let numbers: Vec<&str> = card.split(':').collect();
//         let numbers: &str = numbers[1];
//         let numbers: Vec<&str> = numbers.trim().split('|').collect();

//         let winning_numbers: &str = numbers[0];
//         let ticket_numbers: &str = numbers[1];

//         let winning_numbers: Vec<u8> = winning_numbers
//             .trim()
//             .split_whitespace()
//             .map(|n| n.parse().unwrap())
//             .collect();
//         let ticket_numbers: Vec<u8> = ticket_numbers
//             .trim()
//             .split_whitespace()
//             .map(|n| n.parse().unwrap())
//             .collect();

//         for n in ticket_numbers {
//             if let Some(_) = winning_numbers.iter().find(|&w_n| *w_n == n) {
//                 if card_score == 0 {
//                     card_score = 1;
//                 } else {
//                     card_score = card_score * 2;
//                 }
//             }
//         }

//         score = score + card_score;
//     }

//     dbg!(&score);
//     Ok(())
// }

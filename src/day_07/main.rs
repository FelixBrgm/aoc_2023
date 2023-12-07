use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High_card,
    One_pair,
    Two_pair,
    Three_of_a_kind,
    Full_house,
    Four_of_a_kind,
    Five_of_a_kind,
}
#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: String,
    hand_type: HandType,
    value: u32,
}

impl Hand {
    pub fn new(cards: String, value: u32) -> Self {
        Hand {
            hand_type: Self::get_hand_type_special(&cards),
            cards,
            value,
        }
    }

    fn get_hand_type_special(cards: &str) -> HandType {
        let mut best_hand_type = HandType::High_card;

        Self::get_hand_type_rec(cards, 0, &mut best_hand_type);

        best_hand_type
    }
    fn get_hand_type_rec(cards: &str, i: usize, best_hand_type: &mut HandType) {
        if i >= 5 {
            let new_hand = Self::get_hand_type(cards);
            if new_hand > *best_hand_type {
                *best_hand_type = new_hand;
            }
            return;
        }
        let all_cards: Vec<char> = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
        let c = cards.chars().nth(i);
        if let Some(c) = c {
            if c == 'J' {
                for card in all_cards {
                    let cards = cards.replacen("J", &card.to_string(), 1);
                    Self::get_hand_type_rec(&cards, i + 1, best_hand_type)
                }
            } else {
                Self::get_hand_type_rec(cards, i + 1, best_hand_type);
            }
        }
    }

    fn get_hand_type(cards: &str) -> HandType {
        let cards = String::from(cards);
        let mut map: HashMap<char, u8> = HashMap::new();

        for c in cards.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        let mut card_counts: Vec<u8> = vec![];
        // Check for 5oak
        for (c, n) in &map {
            card_counts.push(*n);
        }

        if card_counts.contains(&5) {
            return HandType::Five_of_a_kind;
        }
        if card_counts.contains(&4) {
            return HandType::Four_of_a_kind;
        }
        if card_counts.contains(&3) {
            if card_counts.contains(&2) {
                return HandType::Full_house;
            }
            return HandType::Three_of_a_kind;
        }
        if card_counts.contains(&2) {
            card_counts.remove(card_counts.iter().position(|n| *n == 2).unwrap());
            if card_counts.contains(&2) {
                return HandType::Two_pair;
            }
            return HandType::One_pair;
        }

        HandType::High_card
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type < other.hand_type {
            return Some(std::cmp::Ordering::Less);
        }
        if self.hand_type > other.hand_type {
            return Some(std::cmp::Ordering::Greater);
        }

        let card_order: Vec<char> = vec![
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        let o_cards: Vec<char> = other.cards.chars().collect();
        for (i, s_card) in self.cards.chars().enumerate() {
            let o_card = o_cards[i];
            if s_card != o_card {
                let s_value = card_order.iter().position(|c| *c == s_card).unwrap();
                let o_value = card_order.iter().position(|c| *c == o_card).unwrap();

                if s_value > o_value {
                    return Some(std::cmp::Ordering::Greater);
                } else {
                    return Some(std::cmp::Ordering::Less);
                }
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_07/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut contents: Vec<&str> = contents.lines().collect();

    let mut hands: Vec<Hand> = vec![];

    for line in contents {
        let line: Vec<&str> = line.split_whitespace().collect();
        let cards = line[0];
        let value = line[1].parse().unwrap();

        hands.push(Hand::new(String::from(cards), value));
    }

    hands.sort();

    let mut score: u128 = 0;
    for (i, hand) in hands.iter().enumerate() {
        score += (i as u128 + 1) * hand.value as u128;
    }
    dbg!(&hands);
    dbg!(&score);

    Ok(())
}
// use std::collections::{HashMap, HashSet};
// use std::fs::File;
// use std::io::{self, Read};
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// enum HandType {
//     High_card,
//     One_pair,
//     Two_pair,
//     Three_of_a_kind,
//     Full_house,
//     Four_of_a_kind,
//     Five_of_a_kind,
// }
// #[derive(Debug, Eq, Ord)]
// struct Hand {
//     cards: String,
//     hand_type: HandType,
//     value: u32,
// }

// impl Hand {
//     pub fn new(cards: String, value: u32) -> Self {
//         Hand {
//             hand_type: Self::get_hand_type(&cards),
//             cards,
//             value,
//         }
//     }

//     fn get_hand_type(cards: &str) -> HandType {
//         let cards = String::from(cards);
//         let mut map: HashMap<char, u8> = HashMap::new();

//         for c in cards.chars() {
//             let count = map.entry(c).or_insert(0);
//             *count += 1;
//         }

//         let mut card_counts: Vec<u8> = vec![];
//         // Check for 5oak
//         for (c, n) in &map {
//             card_counts.push(*n);
//         }

//         if card_counts.contains(&5) {
//             return HandType::Five_of_a_kind;
//         }
//         if card_counts.contains(&4) {
//             return HandType::Four_of_a_kind;
//         }
//         if card_counts.contains(&3) {
//             if card_counts.contains(&2) {
//                 return HandType::Full_house;
//             }
//             return HandType::Three_of_a_kind;
//         }
//         if card_counts.contains(&2) {
//             card_counts.remove(card_counts.iter().position(|n| *n == 2).unwrap());
//             if card_counts.contains(&2) {
//                 return HandType::Two_pair;
//             }
//             return HandType::One_pair;
//         }

//         HandType::High_card
//     }
// }

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if self.hand_type < other.hand_type {
//             return Some(std::cmp::Ordering::Less);
//         }
//         if self.hand_type > other.hand_type {
//             return Some(std::cmp::Ordering::Greater);
//         }

//         let card_order: Vec<char> = vec![
//             '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
//         ];

//         let o_cards: Vec<char> = other.cards.chars().collect();
//         for (i, s_card) in self.cards.chars().enumerate() {
//             let o_card = o_cards[i];
//             if s_card != o_card {
//                 let s_value = card_order.iter().position(|c| *c == s_card).unwrap();
//                 let o_value = card_order.iter().position(|c| *c == o_card).unwrap();

//                 if s_value > o_value {
//                     return Some(std::cmp::Ordering::Greater);
//                 } else {
//                     return Some(std::cmp::Ordering::Less);
//                 }
//             }
//         }
//         Some(std::cmp::Ordering::Equal)
//     }
// }

// impl PartialEq for Hand {
//     fn eq(&self, other: &Self) -> bool {
//         self.hand_type == other.hand_type && self.cards == other.cards
//     }
// }

// fn main() -> io::Result<()> {
//     // Open a file in read-only mode
//     let file_path = "src/day_07/input";
//     let mut file = File::open(file_path)?;

//     // Read the contents of the file into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let mut contents: Vec<&str> = contents.lines().collect();

//     dbg!(&contents);

//     let mut hands: Vec<Hand> = vec![];

//     for line in contents {
//         let line: Vec<&str> = line.split_whitespace().collect();
//         let cards = line[0];
//         let value = line[1].parse().unwrap();

//         hands.push(Hand::new(String::from(cards), value));
//     }

//     hands.sort();

//     let mut score: u128 = 0;
//     for (i, hand) in hands.iter().enumerate() {
//         score += (i as u128 + 1) * hand.value as u128;
//     }

//     dbg!(&score);

//     Ok(())
// }

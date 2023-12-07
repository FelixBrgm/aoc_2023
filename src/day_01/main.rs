use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file_path = "src/day_01/input";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents of the file
    println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let all_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9", "0",
    ];

    let mut digit_to_value: HashMap<&str, usize> = HashMap::new();

    digit_to_value.insert("one", 1);
    digit_to_value.insert("two", 2);
    digit_to_value.insert("three", 3);
    digit_to_value.insert("four", 4);
    digit_to_value.insert("five", 5);
    digit_to_value.insert("six", 6);
    digit_to_value.insert("seven", 7);
    digit_to_value.insert("eight", 8);
    digit_to_value.insert("nine", 9);
    digit_to_value.insert("1", 1);
    digit_to_value.insert("2", 2);
    digit_to_value.insert("3", 3);
    digit_to_value.insert("4", 4);
    digit_to_value.insert("5", 5);
    digit_to_value.insert("6", 6);
    digit_to_value.insert("7", 7);
    digit_to_value.insert("8", 8);
    digit_to_value.insert("9", 9);
    digit_to_value.insert("0", 0);

    let mut result: usize = 0;

    for line in lines {
        let mut first_digit = None;
        let mut first_digit_left_index = None;

        let mut last_digit = None;
        let mut last_digit_right_index = None;

        for (i, keyword) in all_digits.iter().enumerate() {
            if let Some(ind) = line.find(keyword) {
                if let None = first_digit_left_index {
                    first_digit = Some(i);
                    first_digit_left_index = Some(ind);
                } else {
                    if first_digit_left_index > Some(ind) {
                        first_digit = Some(i);
                        first_digit_left_index = Some(ind);
                    }
                }
            }
            // last
            if let Some(ind) = line.rfind(keyword) {
                if let None = last_digit_right_index {
                    last_digit = Some(i);
                    last_digit_right_index = Some(ind);
                } else {
                    if last_digit_right_index < Some(ind) {
                        last_digit = Some(i);
                        last_digit_right_index = Some(ind);
                    }
                }
            }
        }

        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                result = result
                    + digit_to_value[all_digits[first]] * 10
                    + digit_to_value[all_digits[last]]
            }
            _ => {}
        }
    }

    println!("Erg: {}", result);
    Ok(())
}

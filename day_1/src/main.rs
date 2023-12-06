// this is definitely the first time I wrote this and I totally didn't git rm -rf all my code
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn part_1() {
    // open file
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    // save 2-digit numbers from each line
    let mut values: Vec<u32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // record all digits in line
            let mut digits: Vec<u32> = Vec::new();
            for c in line.chars() {
                // save character as u32 if it is a digit
                if c.is_numeric() {
                    digits.push(c.to_digit(10).unwrap());
                }
            }

            values.push(10 * digits[0] + digits.last().unwrap());
        }
    }

    println!("part 1: {:?}", values.iter().sum::<u32>());
}

fn part_2() {
    // open file
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    // save 2-digit number from each line
    let mut values: Vec<u32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // digits in this line
            let mut digits: Vec<u32> = Vec::new();

            // record of characters
            let mut character_string: String = String::new();
            for c in line.chars() {
                // save character
                character_string = character_string + &c.to_string()[..];

                // save actual numbers
                if c.is_numeric() {
                    digits.push(c.to_digit(10).unwrap());
                }

                // look for digit strings
                if character_string.len() >= 3 {
                    match &character_string[character_string.len() - 3..] {
                        "one" => digits.push(1),
                        "two" => digits.push(2),
                        "six" => digits.push(6),
                        _ => {}
                    }
                }
                if character_string.len() >= 4 {
                    match &character_string[character_string.len() - 4..] {
                        "four" => digits.push(4),
                        "five" => digits.push(5),
                        "nine" => digits.push(9),
                        _ => {}
                    }
                }
                if character_string.len() >= 5 {
                    match &character_string[character_string.len() - 5..] {
                        "three" => digits.push(3),
                        "seven" => digits.push(7),
                        "eight" => digits.push(8),
                        _ => {}
                    }
                }
            }

            values.push(10 * digits[0] + digits.last().unwrap());
        }
    }

    println!("part 2: {:?}", values.iter().sum::<u32>());
}

fn main() {
    part_1();
    part_2();
}

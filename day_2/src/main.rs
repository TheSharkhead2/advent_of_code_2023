use std::fs::File;
use std::io::{prelude::*, BufReader};

fn part_1() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let max_counts = vec![12, 13, 14]; // 12 red, 13 green, 14 blue

    let mut possible_games: Vec<u32> = Vec::new();

    // loop through all lines
    for line in reader.lines() {
        if let Ok(line) = line {
            // first split along colon to get game id and rest
            let game_id_counts_split: Vec<&str> = line.split(": ").into_iter().collect();

            // first item contains game id
            let game_id_str: String = game_id_counts_split[0][5..].into();
            let game_id: u32 = game_id_str.parse().unwrap();

            // maximum color counts
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;

            // now split along semi-colons to get each sample
            let samples: Vec<&str> = game_id_counts_split[1].split("; ").into_iter().collect();
            for sample in samples {
                // split along commas
                let color_counts: Vec<&str> = sample.split(", ").into_iter().collect();
                for color in color_counts {
                    // determine which color we are looking at
                    if &color[color.len() - 3..] == "red" {
                        // println!("{:?}", &color[..color.len() - 3]);
                        let red_count: u32 = color[..color.len() - 4].parse().unwrap();

                        // update maximum red count
                        if red_count > max_red {
                            max_red = red_count;
                        }
                    } else if &color[color.len() - 4..] == "blue" {
                        // println!("{:?}", &color[..color.len() - 4]);
                        let blue_count: u32 = color[..color.len() - 5].parse().unwrap();

                        // update max blue count
                        if blue_count > max_blue {
                            max_blue = blue_count;
                        }
                    } else if &color[color.len() - 5..] == "green" {
                        // println!("{:?}", &color[..color.len() - 5]);
                        let green_count: u32 = color[..color.len() - 6].parse().unwrap();

                        if green_count > max_green {
                            max_green = green_count;
                        }
                    }
                }
            }

            if max_counts[0] >= max_red && max_counts[1] >= max_green && max_counts[2] >= max_blue {
                possible_games.push(game_id);
            }
        }
    }

    println!("part 1: {:?}", possible_games.iter().sum::<u32>());
}

fn part_2() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let mut set_powers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // first split along colon to get game id and rest
            let game_id_counts_split: Vec<&str> = line.split(": ").into_iter().collect();

            // maximum color counts
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;

            // now split along semi-colons to get each sample
            let samples: Vec<&str> = game_id_counts_split[1].split("; ").into_iter().collect();
            for sample in samples {
                // split along commas
                let color_counts: Vec<&str> = sample.split(", ").into_iter().collect();
                for color in color_counts {
                    // determine which color we are looking at
                    if &color[color.len() - 3..] == "red" {
                        // println!("{:?}", &color[..color.len() - 3]);
                        let red_count: u32 = color[..color.len() - 4].parse().unwrap();

                        // update maximum red count
                        if red_count > max_red {
                            max_red = red_count;
                        }
                    } else if &color[color.len() - 4..] == "blue" {
                        // println!("{:?}", &color[..color.len() - 4]);
                        let blue_count: u32 = color[..color.len() - 5].parse().unwrap();

                        // update max blue count
                        if blue_count > max_blue {
                            max_blue = blue_count;
                        }
                    } else if &color[color.len() - 5..] == "green" {
                        // println!("{:?}", &color[..color.len() - 5]);
                        let green_count: u32 = color[..color.len() - 6].parse().unwrap();

                        if green_count > max_green {
                            max_green = green_count;
                        }
                    }
                }
            }

            // calculate power
            let power = max_red * max_green * max_blue;
            set_powers.push(power);
        }
    }

    println!("part 2: {:?}", set_powers.iter().sum::<u32>());
}

fn main() {
    part_1();
    part_2();
}

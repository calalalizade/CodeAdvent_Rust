#![allow(unused_variables, dead_code, unused_imports)]

use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Can't open file");

    let mut total_sum_id = 0;
    let mut total_sum_power = 0;

    for line in content.lines() {
        let (sum_id, sum_power) = calculate_sum(&line);
        total_sum_id += sum_id;
        total_sum_power += sum_power;
    }

    println!("{}", total_sum_id);
    println!("{}", total_sum_power);
}

fn calculate_sum(line: &str) -> (u32, u32) {
    let mut id: u32 = 0;

    let mut flag = false;

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for part in line.split(&[',', ':', ';'][..]) {
        let combination: Vec<_> = part.split_whitespace().collect();

        let first_number = combination[0].parse::<u32>().unwrap_or(0);

        match combination.last() {
            Some(&"red") => {
                if first_number > max_red {
                    max_red = first_number;
                }
                if first_number > MAX_RED {
                    flag = true;
                }
            }
            Some(&"green") => {
                if first_number > max_green {
                    max_green = first_number;
                }
                if first_number > MAX_GREEN {
                    flag = true;
                }
            }
            Some(&"blue") => {
                if first_number > max_blue {
                    max_blue = first_number;
                }
                if first_number > MAX_BLUE {
                    flag = true;
                }
            }
            Some(s) => {
                id = s.parse().unwrap_or(0);
            }
            None => println!("No last element"),
        }
    }

    let power = max_blue * max_red * max_green;

    if flag {
        (0, power)
    } else {
        (id, power)
    }
}

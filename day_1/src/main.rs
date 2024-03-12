#![allow(unused_variables, dead_code)]
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Can't open file");

    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total_sum = 0;

    for line in input.lines() {
        let modified_line = letters_to_digits(line.to_string());

        let numeric_digits: Vec<u32> = modified_line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();

        total_sum += numeric_digits.first().unwrap() * 10 + numeric_digits.last().unwrap();
    }
    println!("{}", total_sum);
}

fn letters_to_digits(input_string: String) -> String {
    // let input_string = String::from("7onefour1eighttwo5three");
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // Use fold to replace each digit with its numeric equivalent
    let modified_string = digits
        .iter()
        .fold(input_string, |acc, &digit| { acc.replace(digit, get_numeric_equivalent(digit)) });

    modified_string
}

fn get_numeric_equivalent(digit: &str) -> &str {
    match digit {
        "one" => "o1e",
        "two" => "t2o",
        "three" => "t3e",
        "four" => "f4r",
        "five" => "f5e",
        "six" => "s6x",
        "seven" => "s7n",
        "eight" => "e8t",
        "nine" => "n9e",
        _ => digit,
    }
}

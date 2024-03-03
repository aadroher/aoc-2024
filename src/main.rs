use regex::Regex;
use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn day_1_problem_1(lines: Vec<String>) -> u32 {
    // println!("{:#?}", lines);
    lines
        .iter()
        .map(|line| {
            let chars = line.chars();
            let digits: Vec<u32> = chars
                .filter(|char: &char| char.is_ascii_digit())
                .map(|char: char| char.to_digit(10).unwrap())
                .collect();
            let first_digit = digits.first().expect("Could not find first digit.");
            let last_digit = digits.last().expect("Could not find last digit.");
            first_digit * 10 + last_digit
        })
        .sum()
}

fn parse_digit(digit_string: &str) -> u32 {
    match digit_string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit_string.parse().unwrap(),
    }
}

fn day_1_problem_2(lines: Vec<String>) -> u32 {
    let pattern = r"[0-9]|one|two|three|four|five|six|seven|eight|nine";
    lines
        .iter()
        .map(|line| {
            let re = Regex::new(pattern).unwrap();
            let first_digit_string = re
                .find(&line)
                .expect("Could not find first digit string.")
                .as_str();
            let last_digit_string = re
                .find_iter(line)
                .last()
                .expect("Could not find last digit string.")
                .as_str();
            // println!("{:#?}", first_digit_string);
            // println!("{:#?}", last_digit_string);
            let first_digit = parse_digit(first_digit_string);
            let last_digit = parse_digit(last_digit_string);

            first_digit * 10 + last_digit
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_problem_1_example_test() {
        let input: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|&line| line.to_string())
            .collect();
        let expected: u32 = 142;
        let output = day_1_problem_1(input);
        assert!(output == expected);
    }

    #[test]
    fn day_1_problem_2_example_test() {
        let input: Vec<String> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|&line| line.to_string())
        .collect();
        let expected: u32 = 281;
        let output = day_1_problem_2(input);
        assert!(output == expected);
    }
}

fn load_lines(file_path: &str) -> Vec<String> {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|read_line| match read_line {
            Ok(line) => line,
            Err(error) => {
                panic!("{:#?}", error)
            }
        })
        .collect()
}

fn main() {
    println!("Day 1 Problem 1");
    println!(
        "{:#?}",
        day_1_problem_1(load_lines("./data/day_1/problem_1/input.txt"))
    );
    println!("Day 1 Problem 2");
    println!(
        "{:#?}",
        day_1_problem_2(load_lines("./data/day_1/problem_1/input.txt"))
    );
}

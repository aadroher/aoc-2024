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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|&line| line.to_string())
            .collect();
        let expected: u32 = 142;
        let output = day_1_problem_1(input);
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
    )
}

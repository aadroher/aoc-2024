use std::char;

fn day_1_problem_1(lines: Vec<&str>) -> u32 {
    // println!("{:#?}", lines);
    lines
        .iter()
        .map(|line: &&str| {
            let chars = line.chars();
            println!("{:#?}", chars);
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
        let input: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected: u32 = 142;
        let output = day_1_problem_1(input);
        println!("{}", output);
        assert!(output == expected);
    }
}

fn main() {
    println!("Hello, world!");
}

fn day_1_problem_1(_lines: Vec<&str>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected: i32 = 142;
        assert!(day_1_problem_1(input) == expected)
    }
}

fn main() {
    println!("Hello, world!");
}

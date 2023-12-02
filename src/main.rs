use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("./input_1.txt")
        .expect("Should be able to read the file");

    println!("part1: Sum of numbers={}", part_1(contents))
}

fn part_1(contents: String) -> i64 {
    let mut sum = 0;
    let regex = Regex::new(r"(\d)").unwrap();

    for line in contents.lines() {
        let digits: Vec<String> = regex.find_iter(line)
            .map(|d| d.as_str().to_string())
            .collect();

        let number: i64 = format!("{}{}", digits[0], digits.last().unwrap())
            .parse().unwrap();
        sum += number;
    }
    sum
}

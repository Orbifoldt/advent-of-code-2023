use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIGITS: HashMap<&'static str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

pub fn day01_main() {
    let contents = fs::read_to_string("./input_1.txt")
        .expect("Should be able to read the file");

    println!("part1: Sum of numbers={}", part_1(&contents));
    println!("part2: Sum of numbers={}", part_2(&contents));
}

fn part_1(contents: &String) -> i64 {
    let mut sum = 0;
    let regex = Regex::new(r"\d").unwrap();

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

fn part_2(contents: &String) -> i64 {
    let digits_string = DIGITS.keys().fold(String::new(), |mut acc, elt| {
        if !acc.is_empty() { acc.push_str("|"); }
        acc.push_str(elt);
        acc
    });

    let regex = Regex::new(format!("\\d|{}", digits_string).as_str()).unwrap();
    let reversed_regex = Regex::new(format!("\\d|{}", reverse(&digits_string)).as_str()).unwrap();

    let mut sum = 0;
    for line in contents.lines() {
        let first_digit_string = regex.find(line).unwrap().as_str().to_string();
        let first = to_digit_str(first_digit_string);

        let line_reversed = reverse(&line).as_str().to_owned();
        let last_digit_string = reverse(reversed_regex.find(&line_reversed).unwrap().as_str());
        let last = to_digit_str(last_digit_string);

        let number: i64 = format!("{}{}", first, last)
            .parse().unwrap();
        sum += number;
    }
    sum
}

fn reverse(string: &str) -> String {
    string.chars().rev().collect()
}

fn to_digit_str(input: String) -> String {
    if Regex::new(r"\d").unwrap().is_match(&input) {
        input
    } else {
        let digit = DIGITS.get(input.as_str()).unwrap();
        format!("{digit}")
    }
}

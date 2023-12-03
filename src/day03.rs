use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    println!("day03");

    let schematic = Schematic::new(&fs::read_to_string("./day03/input.txt").unwrap());
    let all_numbers = possible_part_numbers(&schematic);
    let part_numbers: Vec<&Number> = all_numbers.iter()
        .filter(|number| is_part_number(&schematic, number))
        .collect();
    part1(&part_numbers);
    part2(&schematic, &part_numbers)
}

fn part1(part_numbers: &Vec<&Number>) {
    let sum: i32 = part_numbers.iter().map(|number| number.value).sum();
    println!("Sum is {sum}")
}

fn part2(schematic: &Schematic, part_numbers: &Vec<&Number>) {
    let gear_ratios = schematic.content.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == '*' {
                    let neighboring_numbers: Vec<&Number> = part_numbers.iter()
                        .filter(|number| is_neighbor(x as i32, y as i32, number))
                        .map(|refref_number| *refref_number)
                        .collect();
                    if neighboring_numbers.len() == 2 {
                        let ratio = neighboring_numbers[0].value * neighboring_numbers[1].value;
                        Some(ratio)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        });
    let sum: i32 = gear_ratios.sum();
    println!("Sum of gear ratios is {sum}")
}

#[derive(Eq, PartialEq, Debug)]
struct Schematic {
    original: String,
    content: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Number {
    x: usize,
    y: usize,
    length: usize,
    value: i32,
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    static ref NON_SYMBOL_REGEX: Regex = Regex::new(r"\.|\d").unwrap();
}

impl Schematic {
    pub fn new(schematic_string: &str) -> Self {
        Self {
            height: schematic_string.lines().count(),
            width: schematic_string.lines().nth(0).unwrap().len(),
            original: schematic_string.to_string(),
            content: schematic_string.lines().map(|line| line.chars().collect::<Vec<char>>()).collect(),
        }
    }
}

pub fn possible_part_numbers(schematic: &Schematic) -> Vec<Number> {
    schematic.original.lines().enumerate().flat_map(move |(y, line)| {
        NUMBER_REGEX.captures_iter(line).map(move |number_str| {
            Number {
                x: number_str.get(0).unwrap().start(),
                y: y,
                length: number_str[0].len(),
                value: number_str[0].parse().unwrap(),
            }
        })
    }).collect()
}

pub fn is_part_number(schematic: &Schematic, number: &Number) -> bool {
    // println!("Checking {:?}", number);
    let x = number.x as i32;
    let y = number.y as i32;
    let length = number.length as i32;
    let mut neighbors = vec![
        (x - 1, y), (x + length, y),
    ];
    for x_offset in -1..length + 1 {
        neighbors.push((x + x_offset, y + 1));
        neighbors.push((x + x_offset, y - 1));
    }
    neighbors.iter().any(|(x, y)| {
        let x = *x;
        let y = *y;
        if 0 <= x && x < schematic.width as i32 && 0 <= y && y < schematic.height as i32 {
            let char = schematic.content[y as usize][x as usize];
            // println!("Chart at (x,y)=({x},{y}) is '{char}'");
            !char.is_digit(10) && char != '.'
        } else {
            false
        }
    })
}

pub fn is_neighbor(x: i32, y: i32, number: &Number) -> bool {
    let nx = number.x as i32;
    let ny = number.y as i32;
    nx - 1 <= x && x <= nx + number.length as i32 && ny - 1 <= y && y <= ny + 1
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day03::{is_neighbor, is_part_number, Number, possible_part_numbers, Schematic};

    fn load_example() -> Schematic {
        Schematic::new(&fs::read_to_string("./day03/input_example.txt").unwrap())
    }

    #[test]
    fn should_be_able_to_load_schematic() {
        let schematic = load_example();
        assert_eq!(schematic.content[2][3], '5');
        assert_eq!(schematic.width, 10);
        assert_eq!(schematic.height, 10);
    }


    #[test]
    fn should_extract_vec_of_possible_part_numbers() {
        let schematic = load_example();
        let numbers = possible_part_numbers(&schematic);

        assert_eq!(numbers.len(), 10);
        assert_eq!(numbers[0], Number { x: 0, y: 0, length: 3, value: 467 });
        assert_eq!(numbers[1], Number { x: 5, y: 0, length: 3, value: 114 });
        assert_eq!(numbers[2], Number { x: 2, y: 2, length: 2, value: 35 });
        assert_eq!(numbers[5], Number { x: 7, y: 5, length: 2, value: 58 });
        assert_eq!(numbers[9], Number { x: 5, y: 9, length: 3, value: 598 });
    }

    #[test]
    fn should_correctly_check_that_a_number_is_a_valid_part_number() {
        let schematic = load_example();
        let numbers = possible_part_numbers(&schematic);

        assert!(is_part_number(&schematic, &numbers[0]));
        assert!(is_part_number(&schematic, &numbers[2]));
        assert!(is_part_number(&schematic, &numbers[9]));
    }

    #[test]
    fn should_correctly_check_that_a_number_is_not_a_valid_part_number() {
        let schematic = load_example();
        let numbers = possible_part_numbers(&schematic);

        assert!(!is_part_number(&schematic, &numbers[1]));
        assert!(!is_part_number(&schematic, &numbers[5]));
    }

    #[test]
    fn should_be_able_to_determine_when_a_coordinate_is_a_neighbor_to_a_number() {
        let schematic = load_example();
        let numbers = possible_part_numbers(&schematic);

        assert!(is_neighbor(3, 1, &numbers[0]));
        assert!(is_neighbor(3, 1, &numbers[2]));
    }

    #[test]
    fn should_be_able_to_determine_when_a_coordinate_is_not_a_neighbor_to_a_number() {
        let schematic = load_example();
        let numbers = possible_part_numbers(&schematic);

        assert!(!is_neighbor(3, 2, &numbers[0]));
        assert!(!is_neighbor(4, 1, &numbers[0]));
    }
}
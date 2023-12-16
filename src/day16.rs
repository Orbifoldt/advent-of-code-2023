use std::fs;
use std::iter::{repeat, repeat_with};
use std::ops::{Add, Sub};

use itertools::Itertools;
use num::Num;

use crate::day16::Direction::{Down, Left, Right, Up};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day16/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let (field, width, height) = parse(input);
    let total_energized = determine_energization(&field, width, height, (0, 0), Right);
    println!("Part 1: total number of tiles energized is {total_energized}");
    total_energized
}

fn part2(input: &str) -> usize {
    let (field, width, height) = parse(input);

    let max_energized = (0..width).flat_map(|x| vec![((x, 0), Down), ((x, height - 1), Up)])
        .chain((0..height).flat_map(|y| vec![((0, y), Left), ((width - 1, y), Right)]))
        .map(|(start_coord, start_direction)| {
            determine_energization(&field, width, height, start_coord, start_direction)
        })
        .max().unwrap();
    println!("Part 2: maximal number of tiles energized is {max_energized}");
    max_energized
}

#[derive(Clone, Copy, Debug)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn as_power(&self) -> usize { // Up => "0001", Down => "0010", Down => "0100" and Left => "1000"
        2usize.pow(*self as u32)
    }
}

fn determine_energization(field: &Vec<Vec<char>>, width: usize, height: usize, start_coord: (usize, usize), start_direction: Direction) -> usize {
    // For each tile, store if and how we entered it by representing the direction as 4-bit number
    // If we entered a tile from multiple direction we can simply XOR the numbers
    let mut visited = repeat_with(|| repeat(0usize).take(width).collect::<Vec<usize>>()).take(height).collect::<Vec<_>>();
    follow_light(&field, (width, height), &mut visited, start_coord, start_direction);
    // print_field(&field, width, height, &mut visited, true);
    visited.iter().map(|row| row.iter().filter(|&b| *b != 0).count()).sum()
}

fn print_field(field: &Vec<Vec<char>>, width: usize, height: usize, visited: &mut Vec<Vec<usize>>, show_field: bool) {
    for y in 0..height {
        for x in 0..width {
            let c = field[y][x];
            if c == '.' || !show_field {
                if visited[y][x] != 0 { print!("#") } else { print!(".") }
            } else {
                print!("{c}")
            }
        }
        println!()
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    let field = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let height = field.len();
    let width = field[0].len();
    (field, width, height)
}

fn follow_light(field: &Vec<Vec<char>>, (width, height): (usize, usize), visited: &mut Vec<Vec<usize>>, start: (usize, usize), incoming_dir: Direction) {
    let next = next_directions(&field, incoming_dir, start.0, start.1);
    visited[start.1][start.0] = incoming_dir.as_power();

    // Initially below was implemented recursively, but that gave a stack overflow. So we just create
    // our own stack! Might contain duplicates, but that doesn't matter really...
    let mut argument_stack: Vec<((usize, usize), Direction)> =
        next.iter().map(|&next_dir| (start, next_dir)).collect::<Vec<_>>();

    while let Some((current, direction)) = argument_stack.pop() {
        if let Some((next_x, next_y)) = next_coord(current, direction, (width, height)) {
            let next = visited[next_y][next_x];
            if next & (direction.as_power()) != 0 {  // Bitwise AND to check if we came this way already
                continue;
            } else {
                visited[next_y][next_x] = next ^ (direction.as_power());
            }

            let next_directions = next_directions(field, direction, next_x, next_y);
            for next_direction in next_directions {
                argument_stack.push(((next_x, next_y), next_direction))
            }
        } else {
            continue;  // out of bounds
        }
    }
}

fn next_coord<T: Num + PartialOrd + Clone>((x, y): (T, T), direction: Direction, (width, height): (T, T)) -> Option<(T, T)> {
    match direction {
        Left => if x > T::zero() { Some((x - T::one(), y)) } else { None },
        Right => if x < width - T::one() { Some((x + T::one(), y)) } else { None },
        Up => if y > T::zero() { Some((x, y - T::one())) } else { None },
        Down => if y < height - T::one() { Some((x, y + T::one())) } else { None }
    }
}

fn next_directions(field: &Vec<Vec<char>>, direction: Direction, next_x: usize, next_y: usize) -> Vec<Direction> {
    match field[next_y][next_x] {
        '.' => vec![direction],
        '/' => match direction {
            Left => vec![Down],
            Right => vec![Up],
            Up => vec![Right],
            Down => vec![Left],
        }
        '\\' => match direction {
            Left => vec![Up],
            Right => vec![Down],
            Up => vec![Left],
            Down => vec![Right],
        }
        '-' => match direction {
            Up | Down => vec![Left, Right],
            _ => vec![direction],
        }
        '|' => match direction {
            Left | Right => vec![Up, Down],
            _ => vec![direction],
        }
        _ => panic!("Invalid char encountered!")
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day16::{part1, part2};

    #[test]
    fn part_1_example() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(part1(input), 46)
    }

    #[test]
    fn part_1_entering_a_loop() {
        let input = r"
...\......
..........
./.-..\...
..........
.\..../...
..........";
        assert_eq!(part1(input), 19)
    }

    #[test]
    fn part_1_first_char_is_special() {
        let input = r"
/.........
..........";
        assert_eq!(part1(input), 1)
    }

    #[test]
    fn part_1_actual_input_regression_test() {
        assert_eq!(part1(&fs::read_to_string("./inputs/day16/input.txt").unwrap()), 7111)
    }

    #[test]
    fn part_2_example() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(part2(input), 51)
    }
}
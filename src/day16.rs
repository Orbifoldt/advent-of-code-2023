use std::fs;
use std::iter::{repeat, repeat_with};
use std::ops::{Add, Sub};

use itertools::Itertools;
use num::Num;

use crate::common::{Direction, next_coord};
use crate::day16::Direction::{East, North, South, West};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day16/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let (field, width, height) = parse(input);
    let total_energized = determine_energization(&field, width, height, (0, 0), East);
    println!("Part 1: total number of tiles energized is {total_energized}");
    total_energized
}

fn part2(input: &str) -> usize {
    let (field, width, height) = parse(input);

    let max_energized = (0..width).flat_map(|x| vec![((x, 0), South), ((x, height - 1), North)])
        .chain((0..height).flat_map(|y| vec![((0, y), West), ((width - 1, y), East)]))
        .map(|(start_coord, start_direction)| {
            determine_energization(&field, width, height, start_coord, start_direction)
        })
        .max().unwrap();
    println!("Part 2: maximal number of tiles energized is {max_energized}");
    max_energized
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
    visited[start.1][start.0] = incoming_dir.as_power_of_2();

    // Initially below was implemented recursively, but that gave a stack overflow. So we just create
    // our own stack! Might contain duplicates, but that doesn't matter really...
    let mut argument_stack: Vec<((usize, usize), Direction)> =
        next.iter().map(|&next_dir| (start, next_dir)).collect::<Vec<_>>();

    while let Some((current, direction)) = argument_stack.pop() {
        if let Some((next_x, next_y)) = next_coord(current, direction, (width, height)) {
            let next = visited[next_y][next_x];
            if (next & direction.as_power_of_2()) != 0 {  // Bitwise AND to check if we came this way already
                continue;
            } else {
                visited[next_y][next_x] = next ^ (direction.as_power_of_2());
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

fn next_directions(field: &Vec<Vec<char>>, direction: Direction, next_x: usize, next_y: usize) -> Vec<Direction> {
    match field[next_y][next_x] {
        '.' => vec![direction],
        '/' => match direction {
            West => vec![South],
            East => vec![North],
            North => vec![East],
            South => vec![West],
        }
        '\\' => match direction {
            West => vec![North],
            East => vec![South],
            North => vec![West],
            South => vec![East],
        }
        '-' => match direction {
            North | South => vec![West, East],
            _ => vec![direction],
        }
        '|' => match direction {
            West | East => vec![North, South],
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
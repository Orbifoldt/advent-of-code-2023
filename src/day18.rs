use std::{fs, vec};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use crate::common::{Direction, next_coord};
use crate::common::Direction::{East, North, South, West};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day18/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let instructions = parse_pt1(input);
    let dug_squares = count_interior_squares(&instructions);
    println!("Part 1: dug {dug_squares} squares");
    dug_squares
}

fn part2(input: &str) -> usize {
    let instructions = parse_pt2(input);
    let dug_squares = gauss_formula(&instructions);
    println!("Part 1: dug {dug_squares} squares");
    dug_squares.unsigned_abs()
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Instruction {
    direction: Direction,
    length: usize,
}

fn parse_pt1(input: &str) -> Vec<Instruction> {
    input.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            Instruction {
                direction: match split[0] {
                    "U" => North,
                    "R" => East,
                    "D" => South,
                    "L" => West,
                    _ => panic!("Invalid color received!")
                },
                length: split[1].parse().unwrap(),
            }
        })
        .collect()
}

fn count_interior_squares(instructions: &Vec<Instruction>) -> usize {
    let size = 600;
    let mut board = (0..size).map(|_| (0..size).map(|_| false).collect::<Vec<bool>>()).collect::<Vec<_>>();

    let mut x = size / 2;
    let mut y = size / 2;
    for instruction in instructions {
        let coords = match instruction.direction {
            North => (0..=instruction.length).map(|dy| (x, y - dy)).collect::<Vec<(usize, usize)>>(),
            East => (0..=instruction.length).map(|dx| (x + dx, y)).collect::<Vec<(usize, usize)>>(),
            South => (0..=instruction.length).map(|dy| (x, y + dy)).collect::<Vec<(usize, usize)>>(),
            West => (0..=instruction.length).map(|dx| (x - dx, y)).collect::<Vec<(usize, usize)>>(),
        };
        let mut coords = coords.iter();
        while let Some(&(nx, ny)) = coords.next() {
            board[ny][nx] = true;
            (x, y) = (nx, ny);
        }
    }
    size * size - flood_fill((0, 0), size, size, |&(x, y)| !board[y][x])
}

fn flood_fill(start: (usize, usize), width: usize, height: usize, is_in: impl Fn(&(usize, usize)) -> bool) -> usize {
    let mut coords_to_check = vec![start];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while !coords_to_check.is_empty() {
        let current = coords_to_check.pop().unwrap();
        if !visited.contains(&current) && is_in(&current) {
            visited.insert(current);
            coords_to_check.append(&mut get_neighbors(width, height, current));
        }
    }
    visited.iter().count()
}

fn get_neighbors(width: usize, height: usize, coord: (usize, usize)) -> Vec<(usize, usize)> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().filter_map(|(dx, dy)| {
        let neighbor = (coord.0 as isize + dx, coord.1 as isize + dy);
        if (0..width as isize).contains(&neighbor.0) && (0..height as isize).contains(&neighbor.1) {
            Some((neighbor.0 as usize, neighbor.1 as usize))
        } else {
            None
        }
    }).collect()
}

// See https://en.wikipedia.org/wiki/Shoelace_formula
fn gauss_formula(instructions: &Vec<Instruction>) -> isize {
    let (area, _) = instructions.iter()
        .fold((0, (0, 0)), |(area, (x, y)), instruction| {
            let dist = instruction.length as isize;
            match instruction.direction {  // since vertical motion has no area we just ignore it
                North => (area,           (x, y - dist)),
                East => (area + dist * y, (x + dist, y)),
                South => (area,           (x, y + dist)),
                West => (area - dist * y, (x - dist, y)),
            }
        });

    // The formula actually assumes the centers of the squares, so for each square on the perimeter
    // we undercount by 1/2. Assuming a rectangle, on the four corners we actually undercount by 3/4.
    // If we would consider a more complex (possibly non-convex) shape, we have more of such
    // corners, and also inner-corners that are undercounted by only 1/4; however, all these corner squares
    // actually cancel out, leaving us with a total undercount of 1 for the corner squares.
    // So, we need to add #(squares on the perimeter) / 2 and 1 to our total as final correction
    let edge_squares: isize = instructions.iter().map(|instruction| instruction.length as isize).sum();
    area.abs() + edge_squares / 2 + 1
}

fn parse_pt2(input: &str) -> Vec<Instruction> {
    input.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            let color = split[2];
            let c = &color[7..=7];
            let num = &color[2..7];
            Instruction {
                direction: match c {
                    "3" => North,
                    "0" => East,
                    "1" => South,
                    "2" => West,
                    _ => panic!("Invalid color received!")
                },
                length: usize::from_str_radix(num, 16).unwrap(),
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::day18::{part1, part2};

    #[test]
    fn part_1_dummy_example() {
        let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part1(input), 62)
    }

    #[test]
    fn part_2_example() {
        let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part2(input), 952408144115)
    }
}
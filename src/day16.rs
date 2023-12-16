use std::fs;
use std::iter::{repeat, repeat_with};

use itertools::Itertools;

use crate::day16::Direction::{Down, Left, Right, Up};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day16/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> usize {
    let field = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let width = field.len();
    let height = field[0].len();

    // For each tile, store if and how we entered it by representing the direction as 4-bit number
    // If we entered a tile from multiple direction we can simply XOR the numbers
    let mut visited = repeat_with(|| repeat(0b0000usize).take(width).collect::<Vec<usize>>()).take(height).collect::<Vec<_>>();
    visited[0][0] = Right.as_power();

    follow_light(&field, (width, height), &mut visited, (0, 0), Right);

    let total_enegrized = visited.iter().map(|row| row.iter().filter(|&b| *b != 0).count()).sum();
    println!("Part 1: total number of tiles energized is {total_enegrized}");
    total_enegrized
}

fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Clone, Copy)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn as_power(&self) -> usize { // Up => "0001", Down => "0010", Down => "0100" and Left => "1000"
        2usize.pow(*self as u32)
    }
}

fn follow_light(field: &Vec<Vec<char>>, (width, height): (usize, usize), visited: &mut Vec<Vec<usize>>, current: (usize, usize), direction: Direction) {
    if let Some((next_x, next_y)) = next_coord(current, direction, (width, height)) {
        let next = visited[next_y][next_x];
        if next & (direction.as_power()) != 0 {
            return;  // came this way already
        } else {
            visited[next_y][next_x] = next ^ (direction.as_power());
        }
        let next_directions = match field[next_y][next_x] {
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
        };
        for next_direction in next_directions {
            follow_light(field, (width, height), visited, (next_x, next_y), next_direction)
        }
    } else {
        return;  // out of bounds
    }
}

fn next_coord((x, y): (usize, usize), direction: Direction, (width, height): (usize, usize)) -> Option<(usize, usize)> {
    match direction {
        Left => if x > 0 { Some((x - 1, y)) } else { None },
        Right => if x + 1 < width { Some((x + 1, y)) } else { None },
        Up => if y > 0 { Some((x, y - 1)) } else { None },
        Down => if y + 1 < height { Some((x, y + 1)) } else { None }
    }
}


#[cfg(test)]
mod tests {
    use crate::day16::part1;

    #[test]
    fn part_1_example() {
        let input = r".|...\....
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

    // #[test]
    // fn part_2_example() {
    //     let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    //     assert_eq!(part2(input), 145)
    // }
}
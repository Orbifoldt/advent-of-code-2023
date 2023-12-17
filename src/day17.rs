use std::{fs, vec};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use crate::common::{Direction, next_coord};
use crate::common::Direction::{East, North, South, West};
use rayon::prelude::*;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day17/input.txt").unwrap();
    part1(input);
    // part2(input);
}

// 1586 too high in 1558 steps
// 1202 ??? in ~260 steps
// 1149 in 314 steps
// not tried yet: 1128 in 318 steps
fn part1(input: &str) -> usize {
    let field = parse(input);
    // println!("{:?}", field);
    bfs_with_cache_find_path(&field) as usize
}

fn part2(input: &str) -> usize {
    todo!()
}

fn parse(input: &str) -> Vec<Vec<u16>> {
    input.lines().filter(|line| !line.is_empty())
        .map(|line| line.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect::<Vec<u16>>())
        .collect()
}

fn bfs_with_cache_find_path(field: &Vec<Vec<u16>>) -> usize {
    let (width, height) = (field[0].len() as u16, field.len() as u16);
    let start = State { x: 0, y: 0, accumulated_heat: 0, total_straight: 0, last_direction: South };
    let upper_bound = upper_bound_heat(field);
    println!("Finding lowest heat path on field of size {width}x{height}, with upper bound {upper_bound}");

    let mut cache: HashMap<(u16, u16, Direction, u8), usize> = HashMap::new();

    let mut states = vec![start];
    let mut min_heat_incurred = upper_bound;
    let mut steps = 0;

    while !states.is_empty() {
        println!("\nRunning cycle {steps}, with {} states:", states.len());
        let mut new_states = HashSet::new();
        let mut end_states = vec![];
        states.iter()
            .for_each(|state| {
                if state.reached_end(width, height) {
                    end_states.push(state);
                } else {
                    Direction::iterator()
                        .filter(|&direction| direction != &state.last_direction.inverse())
                        .for_each(|&direction| {
                            if let Some((x, y)) = next_coord((state.x, state.y), direction, (width, height)) {
                                let total_straight = if direction == state.last_direction { state.total_straight + 1 } else { 0 };
                                let accumulated_heat = state.accumulated_heat + (field[y as usize][x as usize] as usize);
                                if total_straight < 3 && accumulated_heat <= min_heat_incurred {
                                    let key = (x, y, direction, total_straight);
                                    let state = State { x, y, accumulated_heat, total_straight, last_direction: direction };
                                    if let Some(h) = cache.get_mut(&key) {
                                        if *h > accumulated_heat {
                                            *h = accumulated_heat;
                                            new_states.insert(state);
                                        }
                                    } else {
                                        cache.insert(key, accumulated_heat);
                                        new_states.insert(state);
                                    }
                                }
                            }
                        });
                }
            });

        for end_state in end_states {
            if end_state.accumulated_heat < min_heat_incurred {
                min_heat_incurred = end_state.accumulated_heat
            }
        }

        states = new_states.iter().map(|state| state.clone()).collect();
        states.sort_by_key(|state| (state.accumulated_heat as isize) - (state.distance_from_start().pow(2) as isize));

        steps += 1;
    }
    println!("Reached end with min heat {min_heat_incurred}");
    min_heat_incurred
}

// We can find an upperbound by going diagonally, or if we hit a border by snaking. Snaking on the
// lower border looks like > ^ > v > ^ > v ...
fn upper_bound_heat(field: &Vec<Vec<u16>>) -> usize {
    let dirs = [South, East, North, West];
    let (width, height) = (field[0].len() as u16, field.len() as u16);
    let start = State { x: 0, y: 0, accumulated_heat: 0, total_straight: 0, last_direction: South };

    if height == 1 {
        return field[0].iter().map(|x| *x as usize).sum();
    } else if width == 1 {
        return field.iter().map(|row| row[0] as usize).sum();
    }

    let mut last_dir = East;
    let mut state = start;
    loop {
        if state.reached_end(width, height) {
            break;
        }
        for &dir in dirs.iter().filter(|&dir| dir != &last_dir) {
            if let Some((x, y)) = next_coord((state.x, state.y), dir, (width, height)) {
                state = State { x, y, accumulated_heat: state.accumulated_heat + (field[y as usize][x as usize] as usize), total_straight: 0, last_direction: dir };
                last_dir = dir;
                break;
            }
        }
    }
    state.accumulated_heat
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    x: u16,
    y: u16,
    accumulated_heat: usize,
    total_straight: u8,
    last_direction: Direction,
}

impl State {
    fn distance_from_start(&self) -> usize {
        (self.x as usize) + (self.y as usize)
    }

    fn reached_end(&self, width: u16, height: u16) -> bool {
        self.x == width - 1 && self.y == height - 1
    }
}


#[cfg(test)]
mod tests {
    use crate::day17::{parse, part1, upper_bound_heat};

    #[test]
    fn part_1_dummy_example() {
        let input = r"241";
        assert_eq!(part1(input), 5)
    }

    #[test]
    fn part_1_simple_example() {
        let input = r"
911911191
991119111";
        assert_eq!(part1(input), 11)
    }

    #[test]
    fn part_1_example_with_going_back() {
        let input = r"
9199999999999
9119999911199
9911119119199
9999919199199
9999911199119
9999999999919
9999999999911";
        assert_eq!(part1(input), 24)
    }

    #[test]
    fn part_1_example_with_going_back_upper_bound() {
        let input = r"
9199999999999
9119999911199
9911119119199
9999919199199
9999911199119
9999999999919
9999999999911";
        assert_eq!(upper_bound_heat(&parse(input)), 222)
    }

    #[test]
    fn part_1_example_with_large_board() {
        let input = r"
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111
1111111111111111111111";
        assert_eq!(part1(input), 36)
    }

    #[test]
    fn part_1_example() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part1(input), 102)
    }

    #[test]
    fn part_1_example_upper_bound() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(upper_bound_heat(&parse(input)), 134)
    }
}
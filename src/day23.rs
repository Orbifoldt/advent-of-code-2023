use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day23/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> usize {
    let board = parse(input);
    let (width, height) = (board[0].len(), board.len());
    let start = (1, 0);
    let end = (width - 2, height - 1);

    let mut stack = vec![(HashSet::from([start]), start)];
    let mut max_dist = 0;

    while let Some((path, current)) = stack.pop() {
        for next in next_tiles(&board, current) {
            if next == end {
                max_dist = max(max_dist, path.len())
            } else if !path.contains(&next) {
                let mut new_path = path.clone();
                new_path.insert(next);
                stack.push((new_path, next));
            }
        }
    }

    println!("Part 1: longest walk you can take from {:?} to {:?} has length {max_dist}", start, end);
    max_dist
}



fn part2(input: &str) -> usize {
    todo!()
}


fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

// Returns valid tiles to step onto from the current position
fn next_tiles(field: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    if y == 0 {
        return [(x + 1, y), (x - 1, y), (x, y + 1)].into_iter()
            .filter(|(x, y)| field[*y][*x] != '#')
            .collect_vec();
    }
    // We assert here that the input is "nice", e.g. no slopes pointing into walls, and all neighbors within bounds
    let cur = field[y][x];
    match cur {
        '>' => vec![(x + 1, y)],
        'v' => vec![(x, y + 1)],
        '<' => vec![(x - 1, y)],
        '^' => vec![(x, y - 1)],
        _ => [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
            .filter(|(x, y)| field[*y][*x] != '#')
            .collect_vec()
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day23::part1;

    #[test]
    fn part_1_simple_example() {
        let input = r"#.#####################
#.......###############
#######.###############
#######...............#
#####################.#";
        assert_eq!(part1(input), 24)
    }

    #[test]
    fn part_1_example_1() {
        let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part1(input), 94)
    }

    #[test]
    fn part_1_input() {
        let input = &fs::read_to_string("./inputs/day23/input.txt").unwrap();
        assert_eq!(part1(input), 2306)
    }
}
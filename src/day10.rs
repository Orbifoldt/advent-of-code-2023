use std::{fmt, fs};
use std::fmt::{Formatter, write};
use std::slice::Iter;
use std::str::FromStr;
use crate::day10::Direction::{E, N, W, S};
use crate::day10::NextStep::{DeadEnd, Continue, Start};
use crate::day10::Tile::{EW, GROUND, NE, NS, NW, START, SE, SW};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day10/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> i64 {
    let map = parse(input);
    let loop_length = find_loop_length(&map);
    println!("Part 1: number of steps farthest from start is {}", loop_length / 2);
    loop_length / 2
}

fn part2(input: &str) -> i64 {
    todo!()
}

type Map = Vec<Vec<Tile>>;

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().map(Tile::from_char).collect()).collect()
}

fn find_loop_length(map: &Map) -> i64 {
    let (start_x, start_y) = map.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate()
            .find_map(|(x, c)| if c == &START { Some(x) } else { None })
            .map(|x| (x, y))
    }).expect("Should contain a starting pipe 'S'!");

    let length = Tile::pipes_iter().filter_map(|start_tile| {
        println!("\n\nTrying with start tile {start_tile}:");
        let result = start_tile.to_string().chars()
            .filter_map(|c| {
                let mut heading = Direction::from_str(c.to_string().as_str()).unwrap();
                let mut coord = (start_x, start_y);
                let mut tile = *start_tile;
                println!("\n  Going in {heading} direction from start:");

                let mut done = false;
                let mut length = 0;
                let mut the_outcome: Option<NextStep> = None;

                while the_outcome.is_none() {
                    println!("  - Currently at ({}, {}) which is a {tile} tile, heading {heading}", coord.0, coord.1);
                    let outcome = next_coord(map, coord, heading, *start_tile);
                    match outcome {
                        DeadEnd => {
                            println!("  Hit dead end, terminating...");
                            done = true;
                            the_outcome = Some(DeadEnd)
                        }
                        Start => {
                            println!("  Found the start!");

                            done = true;
                            the_outcome = Some(Start)
                        }
                        Continue((t, c, h)) => { (tile, coord, heading) = (t, c, h); }
                    };
                    length += 1;
                };
                match the_outcome {
                    Some(Start) => Some(length),
                    _ => None,
                }
            })
            .next();
        result
    }).next();

    length.unwrap()
}


fn next_coord(map: &Map, current: (usize, usize), current_heading: Direction, start_tile_replacement: Tile) -> NextStep {
    let width = map.first().unwrap().len();
    let (x, y) = current;
    if (current_heading == W && x == 0) || (current_heading == N && y == 0)
        || (current_heading == E && x > width) || (current_heading == S && y >= map.len()) {
        return DeadEnd;
    }
    let (next_x, next_y) = match current_heading {
        N => (x, y - 1),
        E => (x + 1, y),
        S => (x, y + 1),
        W => (x - 1, y),
    };
    let next_tile = map.get(next_y).unwrap().get(next_x).unwrap();
    if next_tile == &GROUND {
        DeadEnd
    } else {
        let next_heading = if next_tile == &START {
            next_heading(start_tile_replacement, current_heading)
        } else {
            next_heading(*next_tile, current_heading)
        };
        if next_heading.is_none() {
            DeadEnd
        } else {
            let next_heading = next_heading.unwrap();
            let mut current_tile = map.get(y).unwrap().get(x).unwrap();
            if current_tile == &START {
                current_tile = &start_tile_replacement;
            }
            if current_tile.connects_to(next_tile, current_heading){
                if next_tile == &START {
                    Start
                } else {
                    Continue((*next_tile, (next_x, next_y), next_heading))
                }
            } else {
                println!("  Can't go from {current_tile} tile in {current_heading} to a {next_tile} tile.");
                DeadEnd
            }

        }
    }
}

enum NextStep {
    DeadEnd,
    Start,
    Continue((Tile, (usize, usize), Direction)),
}


fn next_heading(tile: Tile, current_heading: Direction) -> Option<Direction> {
    if tile == START {
        return None;
    }
    let coming_from = current_heading.inverse();
    let outgoing_heading_string = tile.to_string().replace(&coming_from.to_string(), "");
    Direction::from_str(outgoing_heading_string.as_str()).ok()
}


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction { N, E, S, W }

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(N),
            "E" => Ok(E),
            "S" => Ok(S),
            "W" => Ok(W),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Tile { NS, EW, NE, NW, SW, SE, GROUND, START }

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            'S' => START,
            _ => GROUND,
        }
    }

    fn pipes_iter() -> Iter<'static, Tile> {
        static TILES: [Tile; 6] = [NS, EW, NE, NW, SW, SE];
        TILES.iter()
    }

    fn connects_to(&self, other: &Tile, heading: Direction) -> bool {
        if !self.to_string().contains(&heading.to_string()) {
            dbg!(format!("Direction {heading} invalid to go from {self} ..."));
            return false
        }
        return other.to_string().contains(&heading.inverse().to_string())
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day10::{find_loop_length, next_heading, parse, part1, part2};
    use crate::day10::Direction::{E, N, S, W};
    use crate::day10::Tile::{EW, GROUND, NE, NS, NW, START, SE, SW};

    #[test]
    fn should_correctly_determine_next_heading() {
        assert_eq!(next_heading(NS, S), Some(S));
        assert_eq!(next_heading(NS, N), Some(N));
        assert_eq!(next_heading(NE, S), Some(E));
        assert_eq!(next_heading(NE, W), Some(N));
        assert_eq!(next_heading(SW, N), Some(W));
        assert_eq!(next_heading(SW, E), Some(S));
    }

    #[test]
    fn should_correctly_check_if_tiles_connect() {
        assert_eq!(NS.connects_to(&NW, S), true);
        assert_eq!(NS.connects_to(&NS, S), true);
        assert_eq!(NS.connects_to(&NE, S), true);
        assert_eq!(NS.connects_to(&EW, S), false);
        assert_eq!(NS.connects_to(&SE, S), false);
        assert_eq!(NS.connects_to(&SW, S), false);

        assert_eq!(SE.connects_to(&EW, E), true);
    }


    #[test]
    fn should_find_loop_length_pt1_example1() {
        let map = parse(r".....
.S-7.
.|.|.
.L-J.
.....");
        let loop_length = find_loop_length(&map);
        assert_eq!(loop_length, 8);
    }

    #[test]
    fn should_find_loop_length_pt1_example2() {
        let map = parse(r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
        let loop_length = find_loop_length(&map);
        assert_eq!(loop_length, 16);
    }

    #[test]
    fn should_find_loop_length_pt1_example2_with_additional_unconnected_pipes() {
        let map = parse(r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        let loop_length = find_loop_length(&map);
        assert_eq!(loop_length, 16);
    }

    #[test]
    fn part1_should_pass() {
        let x= part1(&fs::read_to_string("./inputs/day10/input.txt").unwrap());
        assert_eq!(x, 7066);
    }
}


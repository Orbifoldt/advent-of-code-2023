use std::{fmt, fs};
use std::collections::HashSet;
use std::fmt::Formatter;
use std::slice::Iter;
use std::str::FromStr;

use crate::day10::Direction::{E, N, S, W};
use crate::day10::NextStep::{Continue, DeadEnd, Start};
use crate::day10::Tile::{EW, GROUND, NE, NS, NW, SE, START, SW};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day10/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> i64 {
    let map = parse(input);
    let (_, loop_coords) = find_loop_coords(&map);
    println!("Part 1: number of steps farthest from start is {}", loop_coords.len() / 2);
    (loop_coords.len() / 2) as i64
}

fn part2(input: &str) -> i64 {
    let (sx, sy) = input.lines().enumerate().filter_map(|(y, line)| line.find('S').map(|x| (x, y))).next().unwrap();
    let mut map = parse(input);
    let (start_tile, loop_coords) = find_loop_coords(&map);
    map[sy][sx] = start_tile;

    let loop_coords: HashSet<&Coord<usize>> = loop_coords.iter().collect();
    let mut enhanced = enhance(&map, &loop_coords);

    // Find and fill the outside tiles. This works since we know there is one unique loop of pipe.
    // We already removed all other tiles during the enlarging/enhancing step, so can't have
    // isolated area's other than the internal part of the loop.
    let outside = flood_fill((0, 0), 3 * map_width(&map), 3 * map_height(&map), |(x, y)| !enhanced[*y][*x]);
    outside.iter().for_each(|(x, y)| enhanced[*y][*x] = true);

    let mut inside_count = 0;
    for x in 0..map_width(&map) {
        for y in 0..map_height(&map) {
            let is_filled = (0..3).any(|dy| (0..3).any(|dx| enhanced[3*y + dy][3*x + dx]));
            if !is_filled { inside_count += 1; }
        }
    }
    println!("Part 2: number of enclosed tiles is {inside_count}");
    inside_count as i64
}

// Replaces each tile with a 3x3 version. If tile is part of the loop we enlarge the pipe, else we
// just put in an empty 3x3 tile (we don't care about loose pipes anywhere)
fn enhance(map: &Map, loop_coords: &HashSet<&Coord<usize>>) -> Vec<Vec<bool>> {
    let mut new_map = (0..3 * map_height(map)).map(|x| {
        (0..3 * map_width(map)).map(|y| false).collect()
    }).collect::<Vec<Vec<bool>>>();

    // Set 3x3 versions of tiles appearing in the loop
    for &&(x, y) in loop_coords {
        let (center_x, center_y) = (3 * x + 1, 3 * y + 1);
        new_map[center_y][center_x] = true;

        let tile = get_tile(map, (x, y));
        tile.to_string().chars().for_each(|c| {
            let direction = Direction::from_str(c.to_string().as_str()).unwrap();
            let (dx, dy) = direction.coord_offset();
            new_map[(center_y as isize + dy) as usize][(center_x as isize + dx) as usize] = true;
        })
    }
    new_map
}

type Coord<T> = (T, T);

fn flood_fill(start: Coord<usize>, width: usize, height: usize, is_in: impl Fn(&Coord<usize>) -> bool) -> HashSet<Coord<usize>> {
    let mut coords_to_check: Vec<Coord<usize>> = vec![start];
    let mut visited: HashSet<Coord<usize>> = HashSet::new();

    while !coords_to_check.is_empty() {
        let current = coords_to_check.pop().unwrap();
        if !visited.contains(&current) && is_in(&current) {
            visited.insert(current);
            coords_to_check.append(&mut get_neighbors(width, height, current));
        }
    }
    visited
}

fn get_neighbors(width: usize, height: usize, coord: Coord<usize>) -> Vec<Coord<usize>> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().filter_map(|(dx, dy)| {
        let neighbor = (coord.0 as isize + dx, coord.1 as isize + dy);
        if (0..width as isize).contains(&neighbor.0) && (0..height as isize).contains(&neighbor.1) {
            Some((neighbor.0 as usize, neighbor.1 as usize))
        } else {
            None
        }
    }).collect()
}

type Map = Vec<Vec<Tile>>;

fn get_tile(map: &Map, coord: (usize, usize)) -> Tile {
    *map.get(coord.1).map(|row| row.get(coord.0)).flatten().unwrap()
}

fn map_width(map: &Map) -> usize {
    map.first().unwrap().len()
}

fn map_height(map: &Map) -> usize {
    map.len()
}

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().map(Tile::from_char).collect()).collect()
}

fn find_loop_coords(map: &Map) -> (Tile, Vec<(usize, usize)>) {
    let (start_x, start_y) = map.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate()
            .find_map(|(x, c)| if c == &START { Some(x) } else { None })
            .map(|x| (x, y))
    }).expect("Should contain a starting pipe 'S'!");

    let loop_coords = Tile::pipes_iter().filter_map(|start_tile| {
        // println!("\n\nTrying with start tile {start_tile}:");
        let result = start_tile.to_string().chars()
            .filter_map(|c| {
                // todo: start tile must be connected on both sides, so we can just find any
                //       matching pipes, and then for each pipe just try a single direction

                let mut heading = Direction::from_str(c.to_string().as_str()).unwrap();
                let mut coord = (start_x, start_y);
                let mut tile = *start_tile;

                let mut loop_coords: Vec<(usize, usize)> = vec![];
                loop {
                    loop_coords.push(coord);
                    let outcome = next_coord(map, coord, heading, *start_tile);
                    match outcome {
                        DeadEnd => { return None; }
                        Start => { return Some((*start_tile, loop_coords)); }
                        Continue((t, c, h)) => { (tile, coord, heading) = (t, c, h); }
                    };
                }
                panic!("Unreachable")
            })
            .next();
        result
    }).next();

    loop_coords.unwrap()
}


fn next_coord(map: &Map, current: (usize, usize), current_heading: Direction, start_tile_replacement: Tile) -> NextStep {
    let (current_x, current_y) = current;
    if (current_heading == W && current_x == 0) || (current_heading == N && current_y == 0)
        || (current_heading == E && current_x > map_width(map))
        || (current_heading == S && current_y >= map_height(map)) {
        return DeadEnd;
    }

    let next_coord = move_from(current_x, current_y, current_heading);
    let next_tile = get_tile(map, next_coord);
    if next_tile == GROUND {
        DeadEnd
    } else {
        let next_heading = if next_tile == START {
            next_heading(start_tile_replacement, current_heading)
        } else {
            next_heading(next_tile, current_heading)
        };
        if next_heading.is_none() {
            DeadEnd
        } else {
            let next_heading = next_heading.unwrap();
            let mut current_tile = get_tile(map, current);
            if current_tile == START {
                current_tile = start_tile_replacement;
            }

            if (next_tile == START && current_tile.connects_to(&start_tile_replacement, current_heading))
                || current_tile.connects_to(&next_tile, current_heading) {
                if next_tile == START {
                    Start
                } else {
                    Continue((next_tile, next_coord, next_heading))
                }
            } else {
                println!("  Can't go from {current_tile} tile in {current_heading} to a {next_tile} tile.");
                DeadEnd
            }
        }
    }
}

fn move_from(x: usize, y: usize, heading: Direction) -> (usize, usize) {
    match heading {
        N => (x, y - 1),
        E => (x + 1, y),
        S => (x, y + 1),
        W => (x - 1, y),
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

    fn coord_offset(&self) -> Coord<isize> {
        match self {
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
            W => (-1, 0),
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [N, E, S, W];
        DIRECTIONS.iter()
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
            return false;
        }
        return other.to_string().contains(&heading.inverse().to_string());
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

    use crate::day10::{find_loop_coords, next_heading, parse, part1, part2};
    use crate::day10::Direction::{E, N, S, W};
    use crate::day10::Tile::{EW, NE, NS, NW, SE, SW};

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
        let loop_length = find_loop_coords(&map).1.len();
        assert_eq!(loop_length, 8);
    }

    #[test]
    fn should_find_loop_length_pt1_example2() {
        let map = parse(r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
        let loop_length = find_loop_coords(&map).1.len();
        assert_eq!(loop_length, 16);
    }

    #[test]
    fn should_find_loop_length_pt1_example2_with_additional_unconnected_pipes() {
        let map = parse(r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        let loop_length = find_loop_coords(&map).1.len();
        assert_eq!(loop_length, 16);
    }

    #[test]
    fn part1_should_pass() {
        let x = part1(&fs::read_to_string("./inputs/day10/input.txt").unwrap());
        assert_eq!(x, 7066);
    }

    #[test]
    fn example_part2_1() {
        let x = part2(r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........");
        assert_eq!(x, 4);
    }

    #[test]
    fn example_part2_2() {
        let x = part2(r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...");
        assert_eq!(x, 8);
    }

    #[test]
    fn example_part2_3() {
        let x = part2(r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
        assert_eq!(x, 10);
    }
}


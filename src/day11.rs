use std::collections::HashSet;
use std::convert::identity;
use std::fs;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day11/input.txt").unwrap();
    part1(input);
    part2(input, 1_000_000);
}

fn part1(input: &str) -> i64 {
    let image = parse(input);
    let galaxy_coordinates = galaxy_coords(&image);
    let expanded_coords = expand_coords(&galaxy_coordinates, 2);
    let dist_sum = sum_of_distances(&expanded_coords);

    println!("Part 1: sum of distances of pairs of galaxies is {}", dist_sum);
    dist_sum as i64
}

fn part2(input: &str, expansion_factor: usize) -> i64 {
    let image = parse(input);
    let galaxy_coordinates = galaxy_coords(&image);
    let expanded_coords = expand_coords(&galaxy_coordinates, expansion_factor);
    let dist_sum = sum_of_distances(&expanded_coords);

    println!("Part 2: sum of distances of pairs of galaxies (expansion factor {expansion_factor}) is {}", dist_sum);
    dist_sum as i64
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| line.chars().map(|c| c == '#').collect()).collect()
}

fn galaxy_coords(image: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    image.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().filter_map(move |(x, b)| b.then_some((x, y)))
    ).collect()
}

fn sum_of_distances(galaxy_coordinates: &Vec<(usize, usize)>) -> usize {
    galaxy_coordinates.iter().flat_map(|(ax, ay)|
        galaxy_coordinates.iter().map(|(bx, by)|
            ax.abs_diff(*bx) + ay.abs_diff(*by)
        )
    ).sum::<usize>() / 2  // we count every pair twice: a->b and b->a
}


fn expand_coords(coordinates: &Vec<(usize, usize)>, expansion_factor: usize) -> Vec<(usize, usize)> {
    let (max_x, _) = *coordinates.iter().max_by_key(|(x, y)| x).unwrap();
    let (_, max_y) = *coordinates.iter().max_by_key(|(x, y)| y).unwrap();

    let mut empty_columns = (0..max_x).filter(|&x|
        !coordinates.iter().any(|&(gx, _)| gx == x)
    ).collect::<HashSet<_>>();
    let mut empty_rows = (0..max_y).filter(|&y|
        !coordinates.iter().any(|&(_, gy)| gy == y)
    ).collect::<HashSet<_>>();

    coordinates.iter().map(|(x, y)| {
        let empty_rows_infront = empty_rows.iter().filter(|&row_y| row_y < &y).count();
        let empty_columns_infront = empty_columns.iter().filter(|&col_x| col_x < &x).count();
        (empty_columns_infront * (expansion_factor - 1) + *&x, empty_rows_infront * (expansion_factor - 1) + y)
    }).collect::<Vec<_>>()
}


#[cfg(test)]
mod tests {
    use crate::day11::{part1, part2};

    #[test]
    fn example_part1() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(input), 374)
    }

    #[test]
    fn example_part2_2x() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part2(input, 2), 374)
    }

    #[test]
    fn example_part2_10x() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part2(input, 10), 1030)
    }

    #[test]
    fn example_part2_100x() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part2(input, 100), 8410)
    }
}
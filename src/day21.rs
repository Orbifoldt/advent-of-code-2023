use std::collections::{HashSet, VecDeque};
use std::fs;

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day21/input.txt").unwrap();
    part1(input, 6);
    part2(input);
}

fn part1(input: &str, n: isize) -> usize {
    let (rocks, start) = parse(input);
    let (width, height) = (rocks[0].len() as isize, rocks.len() as isize);

    // Breadth first iteration
    let mut to_check = VecDeque::from([start]);
    for _ in 1..=n {
        let mut reached_this_turn: HashSet<(isize, isize)> = HashSet::new();

        while let Some((x, y)) = to_check.pop_back() {
            if !rocks[y as usize][x as usize] {
                neighbors_pt1((x, y), (width, height)).into_iter()
                    .for_each(|coord| { reached_this_turn.insert(coord); });
            }
        }
        reached_this_turn.into_iter().for_each(|coord| { to_check.push_front(coord); });
    }

    let count = to_check.iter().filter(|&&(x, y)| !rocks[y as usize][x as usize]).count();

    println!("Part 1: In {n} steps we can reach {count} tiles.");
    count
}

fn part2(input: &str) -> isize {
    // Since the grid is 2D, the number of steps will scale quadratically, and since the garden is self-repeating,
    // the number of steps will be following a quadratic sequence: s[n] = a*n^2 + b*n + c
    // Also notice that the required number of steps is quite special
    let total_steps = 26501365; // 26501365 = 65 + 202300*131
    // here 65 is number of steps required to reach edge of the first garden, then 131 to reach end of subsequent garden

    let (rocks, start) = parse(input);
    let (width, height) = (rocks[0].len() as isize, rocks.len() as isize);
    assert_eq!(width, 131);
    assert_eq!(height, 131);

    let mut check_points = vec![];  // capture the number of steps at regular intervals 65 + 131*n
    let mut to_check = VecDeque::from([start]);
    for step in 1..=(65 + 2 * 131) {
        let mut reached_this_turn: HashSet<(isize, isize)> = HashSet::new();

        while let Some((x, y)) = to_check.pop_back() {
            if !is_rock_pt2(&rocks, (x, y), (width, height)) {
                neighbors_pt2((x, y)).into_iter().for_each(|coord| { reached_this_turn.insert(coord); });
            }
        }

        reached_this_turn.into_iter().for_each(|coord| { to_check.push_back(coord); });

        if step % 131 == 65 {
            let count = count_reached_tiles(&rocks, &to_check, width, height);
            println!("In {step} steps we can reach {count} tiles.");
            check_points.push(count);
        }
    }
    let differences = check_points.iter().tuple_windows().map(|(a, b)| *b as isize - *a as isize).collect::<Vec<_>>();
    let difference_differences = differences.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>();
    println!("values: {:?}", check_points);
    println!("differences: {:?}", differences);
    println!("differences of differences: {:?}", difference_differences);
    // let difference_difference_differences = difference_differences.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>();
    // println!("differences of differences of differences: {:?}", difference_difference_differences); // this was all 0's when ran with higher number of steps

    // for a quadratic sequence s[n] = a * n^2 + b * n + c we have:
    // 2a = (s[2] - s[1]) - (s[1] - s[0])  (i.e. difference of differences)
    // a + b = s[1] - s[0]
    // c = s[0]
    let a = difference_differences[0] / 2;  // since we know s is quadratic this is guaranteed to be an integer
    let b = differences[0] - a;
    let c = check_points[0] as isize;
    let s = |n: isize| a * n * n + b * n + c;

    // if f[t] is number of steps, then f[n] = s[ (n-65)/131 ], or in other words, we need to find the number of periods
    // after step 65 too reach our end
    let num_periods_required = (total_steps - 65) / width;  // 202300
    let count = s(num_periods_required);
    println!("Part 2: In {total_steps} steps we can reach {count} tiles."); // 632421652138917
    count
}

fn count_reached_tiles(rocks: &Vec<Vec<bool>>, to_check: &VecDeque<(isize, isize)>, width: isize, height: isize) -> usize {
    to_check.iter().filter(|&&(x, y)| !is_rock_pt2(&rocks, (x, y), (width, height))).count()
}


fn parse(input: &str) -> (Vec<Vec<bool>>, (isize, isize)) {
    let mut start = (0, 0);
    let rocks = input.lines().enumerate().map(|(y, line)|
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => false,
                '#' => true,
                'S' => {
                    start = (x as isize, y as isize);
                    false
                }
                _ => unreachable!("Illegal character received in input")
            }
        }).collect::<Vec<bool>>()
    ).collect::<Vec<Vec<bool>>>();
    (rocks, start)
}

fn neighbors_pt1((x, y): (isize, isize), (width, height): (isize, isize)) -> Vec<(isize, isize)> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .iter()
        .filter_map(|(nx, ny)| if &0 <= nx && nx < &width && &0 <= ny && ny < &height {
            Some((*nx, *ny))
        } else { None })
        .collect()
}

fn neighbors_pt2((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn is_rock_pt2(field: &Vec<Vec<bool>>, (x, y): (isize, isize), (width, height): (isize, isize)) -> bool {
    field[y.rem_euclid(height) as usize][x.rem_euclid(width) as usize]
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day21::{part1, part2};

    #[test]
    fn part_1_example_1() {
        let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part1(input, 6), 16)
    }

    #[test]
    fn part_1_input() {
        let input = &fs::read_to_string("./inputs/day21/input.txt").unwrap();
        assert_eq!(part1(input, 64), 3820)
    }

    // #[test]
    // fn part_2_input() {
    //     let input = &fs::read_to_string("./inputs/day21/input.txt").unwrap();
    //     assert_eq!(part2(input), 632421652138917)
    // }
}
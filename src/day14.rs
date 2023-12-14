use std::fs;
use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day14/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let mut rocks = parse(input);
    roll_north(&mut rocks);
    let load = total_load_north(&rocks);

    println!("Part 1: total load on the north side is {load}");
    load
}

fn part2(input: &str) -> usize {
    let total_cycles = 1000000000;
    let initial_cycles = 100;
    let second_cycles = 300;

    let mut rocks = parse(input);

    // cycle for a while to reach a steady state
    for _ in 0..initial_cycles {
        cycle(&mut rocks);
    }

    // keep a log of the state after each cycle, with this we can find the frequency
    let mut states = vec![];
    for i in 0..second_cycles {
        cycle(&mut rocks);
        states.push(rocks.clone())
    }

    // Crude way to find the frequency: take the last, then check which states are exactly equal
    let last = states.last().unwrap();
    let equilibrium = states.iter().map(|state| state == last).collect::<Vec<_>>();
    // Then using the difference of indices we can determine the wave-length
    let wave_length = equilibrium.iter().enumerate().filter(|(idx, b)| **b)
        .tuple_windows().map(|(a,b)| b.0 - a.0)
        .collect::<Vec<_>>();
    let wave_length_guess = wave_length.last().unwrap();
    if wave_length.iter().all(|f| f == wave_length_guess) {
        //println!("Wave length is {wave_length_guess}");
    } else {
        // If we were to reach this, we would need to tweak the number of cycles we test for
        panic!("No wave-length found!");
    }

    // Now it's just some modular arithmetic to find how often we need to cycle to get in final state
    let cycles_left = total_cycles - initial_cycles - second_cycles;
    let remainder = cycles_left % wave_length_guess;
    for i in 0..remainder {
        cycle(&mut rocks);
    }
    // rocks.iter().for_each(|row| {
    //     row.iter().for_each(|c| print!("{c}"));
    //     println!();
    // });
    let load = total_load_north(&rocks);
    println!("Part 2: after {total_cycles} cycles load on north is {load}");
    load
}
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn set(matrix: &mut Vec<Vec<char>>, (x, y): (usize, usize), value: char) {
    let mut row = matrix.get_mut(y).unwrap();
    row[x] = value
}

fn total_load_north(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter().rev().enumerate().fold(0, |acc, (y, row)| {
        acc + (y+1) *  row.iter().filter(|&c| c == &'O').count()
    })
}

fn cycle(rocks: &mut Vec<Vec<char>>){
    // sometimes copy pasting and tweaking a few things is quicker than trying to be smart
    roll_north(rocks);
    roll_west(rocks);
    roll_south(rocks);
    roll_east(rocks);
}

fn roll_north(matrix: &mut Vec<Vec<char>>){
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == 'O' {
                let mut new_y = y;
                for shift in 1..=y {
                    match matrix[y - shift][x] {
                        '.' => new_y = y-shift,
                        _ => break
                    }
                }
                if new_y != y {
                    set(matrix, (x, new_y), 'O');
                    set(matrix, (x, y), '.');
                }
            }
        }
    }
}

fn roll_south(matrix: &mut Vec<Vec<char>>){
    for y in (0..matrix.len()).rev() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == 'O' {
                let mut new_y = y;
                for try_y in (y+1)..matrix.len() {
                    match matrix[try_y][x] {
                        '.' => new_y = try_y,
                        _ => break
                    }
                }
                if new_y != y {
                    set(matrix, (x, new_y), 'O');
                    set(matrix, (x, y), '.');
                }
            }
        }
    }
}

fn roll_west(matrix: &mut Vec<Vec<char>>){
    for x in 0..matrix[0].len() {
        for y in 0..matrix.len() {
            if matrix[y][x] == 'O' {
                let mut new_x = x;
                for shift in 1..=x {
                    match matrix[y][x-shift] {
                        '.' => new_x = x - shift,
                        _ => break
                    }
                }
                if new_x != x {
                    set(matrix, (new_x, y), 'O');
                    set(matrix, (x, y), '.');
                }
            }
        }
    }
}

fn roll_east(matrix: &mut Vec<Vec<char>>){
    let width = matrix[0].len();
    for x in (0..width).rev() {
        for y in 0..matrix.len() {
            if matrix[y][x] == 'O' {
                let mut new_x = x;
                for try_x in (x+1..width) {
                    match matrix[y][try_x] {
                        '.' => new_x = try_x,
                        _ => break
                    }
                }
                if new_x != x {
                    set(matrix, (new_x, y), 'O');
                    set(matrix, (x, y), '.');
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day14::{part1, part2};

    #[test]
    fn part_1_example_roll_north_and_count_load() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(input), 136)
    }

    #[test]
    fn correctly_determine_reflection_number_horizontally() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part2(input), 64)
    }

}
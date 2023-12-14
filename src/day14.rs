use std::fs;
use std::iter::repeat;
use std::ops::Add;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day14/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> usize {
    let mut rocks = parse(input);
    roll_north(&mut rocks);
    let load = total_load_north(&rocks);

    println!("Part 1: total load on the north side is {load}");
    load
}

// fn part2(input: &str) -> usize {
//     todo!()
//     let mut rocks = parse(input);
//     rocks.iter().for_each(|row| {
//         row.iter().for_each(|c| print!("{c}"));
//         println!();
//     });
//     println!("\n rolling north now...");
//     roll_north(&mut rocks);
//     rocks.iter().for_each(|row| {
//         row.iter().for_each(|c| print!("{c}"));
//         println!();
//     });
//     let load = total_load_north(&rocks);
//
//     println!("Part 2: after {total_cycles} cycles load on north is {load}");
//     load
// }

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn set(matrix: &mut Vec<Vec<char>>, (x, y): (usize, usize), value: char) {
    let mut row = matrix.get_mut(y).unwrap();
    row[x] = value
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

fn total_load_north(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter().rev().enumerate().fold(0, |acc, (y, row)| {
        acc + (y+1) *  row.iter().filter(|&c| c == &'O').count()
    })
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day14::part1;

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
        assert_eq!(part1(input), 136)
    }

}
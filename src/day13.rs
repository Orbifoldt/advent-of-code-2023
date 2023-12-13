use std::fs;
use std::iter::repeat;
use std::ops::Add;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day13/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let sum = parse(input).iter().map(|mirrors|
        find_reflection_number(mirrors, 0).unwrap().0
    ).sum();

    println!("Part 1: sum of all reflection numbers is {sum}");
    sum
}

fn part2(input: &str) -> usize {
    let sum: usize = parse(input).into_iter().map(|mirrors|
        find_reflection_number(&mirrors, 1).unwrap().0
    ).sum();

    println!("Part 2: sum of all reflection numbers with smudges removed is {sum}");
    sum
}

fn parse(input: &str) -> Vec<Vec<Vec<bool>>> {
    input.lines().fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(vec![])
        } else {
            acc.last_mut().unwrap().push(line.chars().map(|c| c == '#').collect::<Vec<bool>>());
        }
        acc
    })
}

fn transpose(mirrors: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = mirrors.first().unwrap().len();
    let height = mirrors.len();

    let mut out: Vec<Vec<bool>> = vec![];
    repeat(vec![]).take(width).for_each(|new_rows| out.push(new_rows));

    for x in 0..width {
        for y in 0..height {
            out.get_mut(x).unwrap().insert(y, mirrors[y][x])
        }
    }
    out
}

fn find_reflection_number(mirrors: &Vec<Vec<bool>>, num_difference: usize) -> Option<(usize, bool, usize)> {
    if let Some(value) = find_horizontal_reflection_index(mirrors, num_difference) {
        return Some((100 * (value + 1), true, value));
    } else if let Some(value) = find_horizontal_reflection_index(&transpose(mirrors), num_difference) {
        return Some((value + 1, false, value));
    }
    None
}

fn find_horizontal_reflection_index(mirrors: &Vec<Vec<bool>>, num_differences: usize) -> Option<usize> {
    for y in 0..mirrors.len() - 1 {
        let mut total_error = 0;
        let mut offset = 0;

        while offset <= y
            && (y + offset + 1) < mirrors.len()
            && total_error <= num_differences
        {
            // The trick here is to just count the number of differences between the reflections
            // For part 1 we want equality, so number of differences should be 0. But in part 2 we
            // want to flip exactly one entry, so we look at that instead
            total_error += num_elements_unequal(&mirrors[y - offset], &mirrors[y + offset + 1]);
            offset += 1;
        }

        if total_error == num_differences {
            return Some(y);
        }
    }
    None
}

fn num_elements_unequal(a: &Vec<bool>, b: &Vec<bool>) -> usize {
    a.iter().zip(b).filter(|(ai, bi)| ai != bi).count()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day13::{part1, part2};

    #[test]
    fn correctly_determine_reflection_number_horizontally() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(input), 400)
    }

    #[test]
    fn correctly_determine_reflection_number_vertically() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(part1(input), 5)
    }

    #[test]
    fn should_find_weighted_sum_of_columns_and_vectors_example_part_1() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(input), 405)
    }

    #[test]
    fn correctly_determine_reflection_number_when_flipping_a_mirror_example1() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(part2(input), 300)
    }

    #[test]
    fn correctly_determine_reflection_number_when_flipping_a_mirror_example2() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part2(input), 100)
    }

    #[test]
    fn should_find_weighted_sum_of_columns_and_vectors_example_part_2() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part2(input), 400)
    }

    #[test]
    fn correctly_determine_reflection_number_when_flipping_a_mirror_bug1() {
        let input = r"#.#.....#.##.##.#
#.#.....#.##.##.#
.....#...#####...
...#.###....#...#
.###..####.#..#.#
#.###.#.#..###..#
..####...##.#.##.
..####...##.#.##.
#.#.#.#.#..###..#
.###..####.#..#.#
...#.###....#...#
.....#...#####...
#.#.....#.##.##.#";
        assert_eq!(part2(input), 700)
    }

    #[test]
    fn correctly_determine_reflection_number_when_flipping_a_mirror_bug2() {
        let input = r".##.#..
.##.###
..#####
#..####
.####..
.#...##
##...##";
        assert_eq!(part2(input), 600)
    }

    #[test]
    fn correctly_determine_reflection_number_when_flipping_a_mirror_bug3() {
        let input = r".#......#..##.###
.###..###.##.####
.##....##.#####.#
#.######.##...##.
##.#..#.##.###..#
#...##...#.#.###.
..##..##.........
#..####.###....#.
.#.####.#.#####..
.##....##.##..##.
###.##.####....##
#..#..#..#..##...
#..####..#####...
#..####..#####...
#..#..#..#..##...";
        assert_eq!(part2(input), 5)
    }
}
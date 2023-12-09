use std::fs;
use crate::common::get_numbers;

pub fn main() {
    let input = &fs::read_to_string("./day09/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> i64 {
    let sum = parse(input).into_iter().map(find_next).sum();
    println!("Part 1: Sum of next sequence elements is: {sum}");
    sum
}

fn part2(input: &str) -> i64 {
    let sum = parse(input).into_iter()
        .map(|mut sequence| { sequence.reverse(); sequence} )
        .map(find_next)
        .sum();
    println!("Part 2: Sum of previous sequence elements is: {sum}");
    sum
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| get_numbers::<i64>(line)).collect()
}

fn find_next(sequence: Vec<i64>) -> i64 {
    let differences: Vec<i64> = sequence.windows(2).map(|window| {
        let [a, b] = window else { panic!("Expected window size equal to 2") };
        b - a
    }).collect();

    *sequence.last().unwrap() + if differences.iter().all(|&x| x == 0) {
        0
    } else {
        find_next(differences)
    }
}


#[cfg(test)]
mod tests {
    use crate::day09::{find_next, part1, part2};

    #[test]
    fn should_find_next_element_in_constant_sequence() {
        assert_eq!(find_next(vec![3, 3, 3, 3, 3]), 3);
    }

    #[test]
    fn should_find_next_element_in_linear_sequence() {
        assert_eq!(find_next(vec![2,4,6,8,10]), 12);
    }

    #[test]
    fn should_find_next_element_in_quadratic_sequence() {
        assert_eq!(find_next(vec![1,4,9,16]), 25);
        assert_eq!(find_next(vec![3,6,11,18]), 27);  // offset by 2
    }

    #[test]
    fn example_should_be_computed_correctly_for_part_1() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sum = part1(input);
        assert_eq!(sum, 114)
    }

    #[test]
    fn example_should_be_computed_correctly_for_part_2() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sum = part2(input);
        assert_eq!(sum, 2)
    }
}
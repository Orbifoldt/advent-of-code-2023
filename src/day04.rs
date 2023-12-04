use std::collections::HashMap;
use std::fs;

use regex::Replacer;

pub fn main() {
    let input = fs::read_to_string("./day04/input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let total_score: i32 = input.lines().map(|card| score(card)).sum();
    println!("Total score is {total_score}")
}

pub fn part2(input: &String) -> u128 {
    let num_cards = input.lines().filter(|line| !line.is_empty()).count();

    let mut card_counts: HashMap<usize, u128> = HashMap::new();
    for i in 1..=num_cards  {
        card_counts.insert(i, 1);
    }

    for (idx, card) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        let idx = idx + 1;
        let num_matching_numbers = number_of_wins(card);
        let card_count = *card_counts.get(&idx).expect("Should have card");
        for i in 1..=num_matching_numbers {
            if idx + i <= num_cards {
                *card_counts.get_mut(&(idx + i)).unwrap() += card_count;
            }
        }
    }

    let total_score = card_counts.values().sum();
    println!("Part 2 score is {total_score}");
    total_score
}

pub fn score(card: &str) -> i32 {
    let num_wins: u32 = number_of_wins(card) as u32;
    if num_wins > 0 { 2i32.pow(num_wins - 1) } else { 0 }
}

pub fn number_of_wins(card: &str) -> usize {
    let (_, all_numbers) = split_first(card, ':').expect("to contain ':'");
    let (winning_str, our_str) = split_first(all_numbers, '|').expect("to contain '|'");

    let winning_nums = get_numbers(winning_str);
    let nums: Vec<i32> = get_numbers(our_str);

    nums.iter().filter(|n| winning_nums.contains(n)).count()
}

fn get_numbers(string: &str) -> Vec<i32> {
    string.split(' ')
        .filter_map(|sub_string| sub_string.parse::<i32>().ok())
        .collect()
}

/// Split a string at the first occurrence of `split_at` and return `Some` pair of the head and tail,
/// or `None` if `split_at` is not present in `string`.
fn split_first(string: &str, split_at: char) -> Option<(&str, &str)> {
    string.find(split_at)
        .map_or(None, |idx| Some((&string[..idx], &string[idx+1..])))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day04::{number_of_wins, part2, score};

    #[test]
    fn should_count_wins_in_card_1() {
        let num = number_of_wins("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(num, 4);
    }

    #[test]
    fn should_count_wins_in_card_2() {
        let num = number_of_wins("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(num, 2);
    }

    #[test]
    fn when_there_are_no_wins_should_return_0() {
        let num = number_of_wins("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(num, 0);
    }

    #[test]
    fn should_return_score_8_for_card_1() {
        let num = score("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(num, 8);
    }

    #[test]
    fn should_return_score_2_for_card_2() {
        let num = score("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(num, 2);
    }

    #[test]
    fn should_return_score_8_for() {
        let num = score("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(num, 0);
    }

    #[test]
    fn should_return_correct_number_of_cards_for_part2() {
        let pt2 = part2(&fs::read_to_string("./day04/input_example.txt").unwrap());
        assert_eq!(pt2, 30)
    }
}
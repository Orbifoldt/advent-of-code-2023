use std::fs;
use std::ops::{Range};
use rayon::prelude::*;
use crate::common::{get_numbers, split_first};
use crate::range_set_theory::{cut_out_many, intersect_range};

pub fn main() {
    let input = &fs::read_to_string("./day05/input.txt").unwrap();
    part1(input);
    part2(input);
}

pub fn part1(input: &str) -> i64 {
    let almanac = parse(input);

    let lowest_location = almanac.seeds.iter()
        .map(|seed_range| almanac.follow(&seed_range.start)).min()
        .unwrap();
    println!("Part 1: minimum location for any seed is {lowest_location}");
    lowest_location
}

pub fn part2(input: &str) -> i64 {
    let almanac = parse_part2(input);

    let lowest_location = almanac.seeds.par_iter()
        .enumerate()
        .flat_map(|(idx, seeds)| {
            println!("Processing range {idx}/{}: [{}, {})...", almanac.seeds.len(),seeds.start, seeds.end);
            (seeds.start..seeds.end)
                .into_par_iter()
                .map(|seed| almanac.follow(&seed))
        })
        .min()
        .unwrap();
    println!("Part 2: minimum location for any seed is {lowest_location}");
    lowest_location
}

pub fn parse(input: &str) -> Almanac {
    let seeds_str = input.lines().find(|_| true).expect("Should have a first line");
    let seeds = get_numbers(split_first(seeds_str, ':').unwrap().1)
        .iter().map(|seed| *seed..(*seed + 1)).collect::<Vec<_>>();

    parse_maps(input, seeds)
}

pub fn parse_part2(input: &str) -> Almanac {
    let seeds_str = input.lines().find(|_| true).expect("Should have a first line");
    let seeds = get_numbers(split_first(seeds_str, ':').unwrap().1).chunks(2)
        .map(|chunk| {
            let [start, range] = chunk else { panic!("Expected even number of seeds") };
            *start..(*start + *range)
        })
        .collect::<Vec<_>>();

    parse_maps(input, seeds)
}

fn parse_maps(input: &str, seeds: Vec<Range<i64>>) -> Almanac {
    input.lines().skip(2)
        .fold(Almanac { seeds, entries: vec![] }, |mut almanac, line| {
            let first_char = line.chars().next().unwrap_or(' ');
            if first_char.is_alphabetic() {
                almanac.entries.push(RangedMap {
                    name: &*line,
                    mapping: vec![],
                });
            } else if first_char.is_numeric() {
                let [destination_start, source_start, range_length] = get_numbers(line)[..] else { panic!("Expected 3 numbers") };
                almanac.entries.last_mut().unwrap()
                    .mapping.push(RangedMapEntry {
                    start: source_start,
                    range_length: range_length,
                    target_start: destination_start,
                });
            }
            almanac
        })
}

struct Almanac<'a> {
    seeds: Vec<Range<i64>>,
    entries: Vec<RangedMap<'a>>,
}

impl<'a> Almanac<'a> {
    pub fn follow(&self, seed: &i64) -> i64 {
        let mut next = *seed;
        for map in self.entries.iter() {
            next = map.get(&next);
        }
        next
    }
}

struct RangedMap<'a> {
    name: &'a str,
    mapping: Vec<RangedMapEntry>,
}

impl<'a> RangedMap<'a> {
    pub fn get(&self, seed: &i64) -> i64 {
        self.mapping.iter().find(|entry|
            &entry.start <= seed && seed < &(entry.start + entry.range_length)
        )
            .map(|entry| {
                let offset_from_start = seed - entry.start;
                entry.target_start + offset_from_start
            })
            .unwrap_or(*seed)
    }

}

struct RangedMapEntry {
    start: i64,
    range_length: i64,
    target_start: i64,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day05::{parse, part1, part2};

    #[test]
    fn should_return_correct_minimum_seed_location() {
        let lowest_location = part1(&fs::read_to_string("./day05/input_example.txt").unwrap());
        assert_eq!(lowest_location, 35)
    }

    #[test]
    fn almanac_should_correctly_map_seed_to_soil() {
        let input = &fs::read_to_string("./day05/input_example.txt").unwrap();
        let almanac = parse(input);
        assert_eq!(almanac.entries[0].get(&79), 81)
    }

    #[test]
    fn should_return_correct_minimum_seed_location_part_2() {
        let lowest_location = part2(&fs::read_to_string("./day05/input_example.txt").unwrap());
        assert_eq!(lowest_location, 46)
    }
}
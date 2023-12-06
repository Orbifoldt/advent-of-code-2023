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

// pub fn part2(input: &str) -> i64 {
//     let almanac = parse_part2(input);
//     // todo!("");
//
//     // let output_ranges = almanac.entries.iter().fold(
//     //     almanac.seeds,
//     //     |things_to_check, map| {
//     //         println!("\n=> {}", map.name);
//     //         things_to_check.iter()
//     //             .flat_map(|input_range| {
//     //                 println!("Calculating output for input range [{},{})", input_range.start, input_range.end);
//     //                 map.apply_to(input_range)
//     //             })
//     //             .collect()
//     //     });
//
//     let output_ranges = almanac.entries.iter().fold(
//         almanac.seeds,
//         |things_to_check, map| {
//             println!("\n=> {}", map.name);
//             things_to_check.iter()
//                 .flat_map(|input_range| {
//                     println!("Calculating output for input range [{},{})", input_range.start, input_range.end);
//                     map.apply_to(input_range)
//                 })
//                 .collect()
//         });
//
//
//
//     let lowest_location = output_ranges.iter().map(|range| range.start).min().unwrap();
//     println!("Part 1: minimum location for any seed is {lowest_location}");
//     lowest_location
// }

pub fn part2(input: &str) -> i64 {
    let almanac = parse_part2(input);
    // let lowest_location = 0(0..100u64).into_par_iter()
    //     .sum();

    // let seeds = almanac.seeds.iter().next().unwrap();
    // let lowest_location = (seeds.start as u64..seeds.end as u64).into_par_iter()
    //     .map(|seed: u64| almanac.follow(&(seed as i64)))
    //     .min()
    //     .unwrap();
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
    // todo!()
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

    pub fn apply_to_old(&self, input_range: Range<i64>) -> Vec<Range<i64>> {
        let intersections = self.mapping.iter()
            .map(|entry| intersect_range(&entry.as_range(), &input_range))
            .collect::<Vec<_>>();
        let input_range_without_intersections = cut_out_many(input_range, &intersections);

        intersections.iter().map(|rng: &Range<i64>| {
            let target_start = self.get(&rng.start);
            (target_start..(target_start + rng.end - rng.start))
        }).into_iter()
            .chain(input_range_without_intersections.into_iter())
            .collect()
    }

    pub fn apply_to(&self, input_range: &Range<i64>) -> Vec<Range<i64>> {
        // self.mapping.iter().map(|map|)

        let mut y = vec![input_range.start..input_range.end];
        y.extend_from_slice(self.mapping.iter().map(|entry| intersect_range(input_range, &entry.as_range())).collect::<Vec<_>>().iter().as_slice());
        let ranges = split_at_many_boundaries(y);
        ranges.iter()
            .map(|rng| {
                let target_start = self.get(&rng.start);
                target_start..(target_start+rng.end - rng.start)
            })
            .collect()
    }

    // pub fn apply_to_many(&self, input_ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    //
    //
    //
    //     let intersections = self.mapping.iter()
    //         .map(|entry| intersect_range(&entry.as_range(), &input_range))
    //         .collect::<Vec<_>>();
    //     let input_range_without_intersections = cut_out_many(input_range, &intersections);
    //
    //     intersections.iter().map(|rng: &Range<i64>| {
    //         let target_start = self.get(&rng.start);
    //         (target_start..(target_start + rng.end - rng.start))
    //     }).into_iter()
    //         .chain(input_range_without_intersections.into_iter())
    //         .collect()
    // }
}

// fn split_at_boundaries(a: Range<i64>, b: Range<i64>) -> Vec<Range<i64>> {
//     let mut endpoints = vec![a.start, a.end, b.start, b.end];
//     endpoints.sort();
//     endpoints.dedup();
//     endpoints.windows(2).map(|window| {
//         let [a,b] = window else { panic!("") };
//         a..b
//     }).collect()
// }
fn split_at_many_boundaries(aa: Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut endpoints = aa.iter().flat_map(|a| [a.start, a.end]).collect::<Vec<_>>();
    endpoints.sort();
    endpoints.dedup();
    endpoints.windows(2).map(|window| {
        let [a,b] = window else { panic!("God is dead") };
        *a..*b
    }).collect()
}





struct RangedMapEntry {
    start: i64,
    range_length: i64,
    target_start: i64,
}

impl RangedMapEntry {
    pub fn as_range(&self) -> Range<i64> {
        self.start..(self.start + self.range_length)
    }

    // the shift that this rangedmapentry applies to its input
    pub fn shift(&self) -> i64 {
        self.target_start - self.start
    }
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
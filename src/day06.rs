use std::fs;
use std::iter::zip;
use std::ops::Range;

use crate::common::{get_numbers, split_first};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day06/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> i64 {
    let prod_beating_times: i64 = parse_part1(input).iter().map(|(time, distance)| {
        let times_interval = times_that_beat_record(time, distance);
        times_interval.end - times_interval.start
    }).product();
    println!("Product of the beating number of times for each race is {prod_beating_times}");
    prod_beating_times
}

fn part2(input: &str) -> i64 {
    let (time, distance) = parse_part2(input);
    let times_interval = times_that_beat_record(&time, &distance);
    let num_winning_times = times_interval.end - times_interval.start;
    println!("Product of the beating number of times for each race is {num_winning_times}");
    num_winning_times
}

fn parse_part1(input: &str) -> Vec<(i64, i64)> {
    let x = input.lines().take(2).map(|line| {
        let (_, num_str) = split_first(line, ':').expect("Contains :");
        get_numbers::<i64>(num_str)
    }).collect::<Vec<_>>();
    let [times, distances] = x.as_slice() else { panic!("Expected exactly 2 input lines") };
    zip(times, distances).map(|(t, d)| (*t, *d)).collect()
}

fn parse_part2(input: &str) -> (i64, i64) {
    let x = input.lines().take(2).map(|line| {
        let (_, num_str) = split_first(line, ':').expect("Contains :");
        num_str.replace(' ', "").parse::<i64>().unwrap()
    }).collect::<Vec<_>>();
    let [time, distance] = x.as_slice() else { panic!("Expected exactly 2 input lines") };
    (*time, *distance)
}

// not used, but just figuring out how to create higher order functions
fn distance_fn(allowed_time: &i64) -> impl Fn(i64) -> i64 {
    let allowed_time_clone = *allowed_time;  // I guess this is how you capture it in the closure...
    move |x| (allowed_time_clone - x) * x
}

fn times_that_beat_record(allowed_time: &i64, record: &i64) -> Range<i64> {
    // We want to solve (allowed_time - t) * t > record. Equality is given at:
    let min_t =  ((*allowed_time as f64) - ((allowed_time.pow(2) - 4*record) as f64).sqrt()) / 2f64;
    let max_t =  ((*allowed_time as f64) + ((allowed_time.pow(2) - 4*record) as f64).sqrt()) / 2f64;

    (min_t.floor() as i64 + 1)..(max_t.ceil() as i64)
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day06::{part1, part2};

    #[test]
    fn should_return_correct_total_number_of_winning_times() {
        let x = part1(&fs::read_to_string("./inputs/day06/input_example.txt").unwrap());
        assert_eq!(x, 288)
    }

    #[test]
    fn should_return_correct_total_number_of_winning_times_part2() {
        let x = part2(&fs::read_to_string("./inputs/day06/input_example.txt").unwrap());
        assert_eq!(x, 71503)
    }
}
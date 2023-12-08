use std::collections::BTreeMap;
use std::fs;
use crate::common::{lcm, split_first};

pub fn main() {
    let input = &fs::read_to_string("./day08/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> i64 {
    let (instructions, map) = parse(input);
    let steps = steps_required(instructions, &map, "AAA", |current| current == "ZZZ");
    println!("For part 1 it took {steps} steps.");
    steps
}

fn part2(input: &str) -> i64 {
    let is_at_end = |node: &str| node.ends_with('Z');
    let (instructions, map) = parse(input);
    let individual_steps = map.keys()
        .filter(|&&node_name| node_name.ends_with('A'))
        .map(|start_node| steps_required(instructions, &map, start_node, is_at_end))
        .collect::<Vec<_>>();
    let steps = individual_steps.iter().copied().reduce(|a, b| { lcm(a, b) }).unwrap();
    println!("For part 2 it took {steps} steps.");
    steps
}

fn steps_required(
    instructions: &str,
    map: &BTreeMap<&str, (&str, &str)>,
    start: &str,
    is_at_end: fn(&str) -> bool
) -> i64 {
    let (mut current, mut steps) = (start, 0);
    for instruction in instructions.chars().cycle() {
        let node = map.get(current).unwrap();
        current = match instruction {
            'L' => node.0,
            _ => node.1,
        };
        steps += 1;
        if is_at_end(current) {
            break
        }
    }
    steps
}

fn parse(input: &str) -> (&str, BTreeMap<&str, (&str, &str)>) {
    let instructions = input.lines().next().unwrap();
    let map: BTreeMap<&str, (&str, &str)> = input.lines().skip(2).map(|line| {
        let (node_name, neighbors) = split_first(line, '=').unwrap();
        let (left, right) = split_first(&neighbors.trim()[1..9], ',').unwrap();
        (node_name.trim(), (left.trim(), right.trim()))
    }).collect();
    (instructions, map)
}


#[cfg(test)]
mod tests {
    use crate::day08::{part1, part2};

    #[test]
    fn example_1_should_be_computed_correctly(){
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let steps = part1(input);
        assert_eq!(steps, 2)

    }

    #[test]
    fn example_2_should_be_computed_correctly(){
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ";
        let steps = part1(input);
        assert_eq!(steps, 6)
    }

    #[test]
    fn example_part_2_should_be_computed_correctly(){
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let steps = part2(input);
        assert_eq!(steps, 6)
    }
}
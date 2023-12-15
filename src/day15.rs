use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day15/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let hash_sum = input.split(',').map(hash).sum();
    println!("Part 1: sum of hashes of input is {hash_sum}");
    hash_sum
}

fn part2(input: &str) -> usize {
    let mut boxes = HashMap::new();
    parse(input, &mut boxes);
    let score = boxes.iter().map(|(k, v)| -> usize{
        (k + 1) * v.iter().enumerate().map(|(idx, lens)| (idx + 1) * lens.focal_length).sum::<usize>()
    })
        .sum();
    println!("Part 2: score is {score}");
    score
}

fn hash(something: &str) -> usize {
    something.to_string().bytes().fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}

fn parse(input: &str, state: &mut HashMap<usize, Vec<Lens>>) {
    input.split(',').for_each(|section| {
        if section.contains('=') {  // insert into or update a box
            let (label, focal_length) = section.split_at(section.find("=").unwrap());
            let box_number = hash(label);
            let lens = Lens { label: label.to_string(), focal_length: focal_length[1..].parse().unwrap() };

            if let Some(content) = state.get_mut(&box_number) {
                let idx = content.iter().position(|lens| lens.label == label);
                if idx.is_some() {  // lens with same label already in box, update focal length
                    content[idx.unwrap()].focal_length = lens.focal_length
                } else {
                    content.push(lens);
                }
            } else {  // box doesn't exist yet
                state.insert(box_number, vec![lens]);
            }
        } else {  // remove lens from box
            let label = section.split('-').next().unwrap();
            let box_number = hash(label);

            state.get_mut(&box_number)
                .map(|content| {
                    content.iter().position(|lens| lens.label == label)
                        .map(|idx| content.remove(idx))
                });
        }
        // println!("After {section}:");
        // state.iter().for_each(|(k, v)| println!("Box {k}: {:?}", v));
        // println!()
    });
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}


#[cfg(test)]
mod tests {
    use crate::day15::{part1, part2};

    #[test]
    fn part_1_example_1() {
        let input = r"HASH";
        assert_eq!(part1(input), 52)
    }

    #[test]
    fn part_1_example_2() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320)
    }

    #[test]
    fn part_2_example() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145)
    }
}
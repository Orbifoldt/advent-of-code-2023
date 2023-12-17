use std::fs;

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day17/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> usize {
    // let (field, width, height) = parse(input);
    // let total_energized = determine_energization(&field, width, height, (0, 0), East);
    // println!("Part 1: total number of tiles energized is {total_energized}");
    // total_energized
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::day17::part1;

    #[test]
    fn part_1_example() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part1(input), 102)
    }


}
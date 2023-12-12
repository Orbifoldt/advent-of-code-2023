use std::fs;
use std::iter::repeat;
use itertools::Itertools;
use crate::common::{get_numbers, split_first};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day12/input.txt").unwrap();
    part1(input);
    // part2(input, 1_000_000);
}

fn part1(input: &str) -> usize {
    let sum_configs = input.lines().map(|line|
        num_valid_configs(line)
    ).sum();
    println!("Part 1: sum of all possible configurations is {sum_configs}");
    sum_configs
}

fn part2(input: &str) -> usize {
    let sum_configs = input.lines().map(|line|
        num_valid_configs(unfold(line).as_str())
    ).sum();
    println!("Part 2: sum of all possible configurations is {sum_configs}");
    sum_configs
}

fn unfold(folded_line: &str) -> String {
    let (config_str, groups_str) = split_first(folded_line, ' ').unwrap();
    let unfolded_config = repeat(config_str).take(5).intersperse("?").collect::<String>();
    let unfolded_groups = repeat(groups_str).take(5).intersperse(",").collect::<String>();
    unfolded_config + " " + &*unfolded_groups

}

fn num_valid_configs(springs_line: &str) -> usize {
    let (config_str, groups_str) = split_first(springs_line, ' ').unwrap();
    let groups = get_numbers::<usize>(groups_str.replace(",", " ").as_str());

    let question_mark_count: u32 = config_str.chars().filter(|c| c == &'?').count() as u32;

    let mut count = 0;
    for i in 0..2usize.pow(question_mark_count) {
        let mut bools: Vec<bool> = vec![];
        for d in 0..question_mark_count {
            bools.push(((i >> d) & 1) == 1)
        }

        let config = config_str.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => bools.pop().unwrap()
            }
        }).collect::<Vec<_>>();

        if matches(config, &groups) {
            count += 1;
        }
    }
    // println!("{springs_line} => {count}");
    count
}

fn matches(spring_config: Vec<bool>, groups: &Vec<usize>) -> bool {
    let mut group_size = 0;
    let mut in_group = false;
    let mut found_groups: Vec<usize> = vec![];
    for b in spring_config {  // entering a new group
        if b && !in_group {
            in_group = true;
            group_size += 1;
        } else if b && in_group {  // inside an existing group
            group_size += 1;
        } else if !b && in_group { // group just ended
            found_groups.push(group_size.clone());
            group_size = 0;
            in_group = false;
        } else { // outside a group
            // continue
        }
    }
    if group_size != 0 {  // still in group at the end of the springs
        found_groups.push(group_size)
    }

    &found_groups == groups
}



#[cfg(test)]
mod tests {
    use crate::day12::{matches, num_valid_configs, part1, part2, unfold};

    #[test]
    fn correctly_determine_which_configs_are_valid(){
        assert_eq!(matches(vec![true, false, true, false, true, true, true], &vec![1,1,3]), true)
    }

    #[test]
    fn correctly_determine_number_of_configurations(){
        assert_eq!(num_valid_configs("???.### 1,1,3"), 1);
        assert_eq!(num_valid_configs(".??..??...?##. 1,1,3"), 4);
        assert_eq!(num_valid_configs("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(num_valid_configs("????.#...#... 4,1,1"), 1);
        assert_eq!(num_valid_configs("????.######..#####. 1,6,5"), 4);
        assert_eq!(num_valid_configs("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn example_part1() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(input), 21)
    }

    #[test]
    fn should_correctly_unfold_a_line(){
        assert_eq!(unfold(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(
            unfold("???.### 1,1,3"),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
        );
    }

    #[test]
    fn example_part2() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part2(input), 506250)
    }
}